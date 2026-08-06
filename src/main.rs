#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod vm;
mod save;
mod util;
mod read;
mod write;
mod ui;
mod db;

use std::{fs::{self, File}, io::Write, path::PathBuf};

use eframe::{egui::{self, text::LayoutJob, Align, CornerRadius, FontSelection, Id, LayerId, Layout, Order, RichText, Style}, epaint::Color32};
use rfd::FileDialog;
use chrono::Local;
use save::save::save::{Save, SaveType};
use ui::{equipment::equipment::equipment, events::events::events, general::general::general, importer::import::character_importer, inventory::inventory::inventory::inventory, menu::menu::{menu, Route}, none::none::none, regions::regions::regions, settings::settings::settings as settings_view, stats::stats::stats};
use vm::{importer::general_view_model::ImporterViewModel, vm::vm::ViewModel};
use crate::write::write::Write as w; 
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "icon/"]
struct Asset;

const WINDOW_WIDTH: f32 = 1920.;
const WINDOW_HEIGHT: f32 = 960.;

fn main() -> Result<(), eframe::Error> {
    // App Icon
    let mut app_icon = egui::IconData::default();
    
    let image = Asset::get("icon.png").expect("Failed to get image data").data;
    let icon = image::load_from_memory(&image).expect("Failed to open icon path").to_rgba8();
    let (icon_width, icon_height) = icon.dimensions();
    app_icon.rgba = icon.into_raw();
    app_icon.width = icon_width;
    app_icon.height = icon_height;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title(format!("ER Save Editor {}", env!("CARGO_PKG_VERSION")))
            .with_inner_size([WINDOW_WIDTH, WINDOW_HEIGHT])
            .with_min_inner_size([1280., 720.])
            .with_icon(app_icon),
        ..Default::default()
    };

    eframe::run_native("ER Save Editor", options, Box::new(|creation_context| {
        let mut fonts = egui::FontDefinitions::default();
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Regular);
        egui_phosphor::add_to_fonts(&mut fonts, egui_phosphor::Variant::Fill);
        creation_context.egui_ctx.set_fonts(fonts);
        let mut visuals = creation_context.egui_ctx.style().visuals.clone();
        let rounding = 3.;
        visuals.window_corner_radius = CornerRadius::default().at_least(rounding as u8);
        visuals.window_highlight_topmost = false;
        creation_context.egui_ctx.set_visuals(visuals);
        Ok(Box::new(App::new(creation_context)))
    }))
}

pub struct App {
    save: Save,
    vm: ViewModel,
    picked_path: PathBuf,
    current_route: Route,
    importer_vm: ImporterViewModel,
    importer_open: bool,
    backup_folder: Option<PathBuf>,
    settings_open: bool,
    show_save_confirm: bool,
    pending_save_path: Option<PathBuf>,
    pending_backup_name: Option<String>,
    save_error: Option<String>,
    save_status: Option<String>,
    zoom: f32,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let zoom = Self::load_zoom(cc).unwrap_or(1.0);
        cc.egui_ctx.set_zoom_factor(zoom);
        Self {
            save: Save::default(),
            picked_path: Default::default(),
            current_route: Route::None,
            vm: ViewModel::default(),
            importer_vm: Default::default(),
            importer_open: Default::default(),
            backup_folder: Self::load_backup_folder(cc),
            settings_open: false,
            show_save_confirm: false,
            pending_save_path: None,
            pending_backup_name: None,
            save_error: None,
            save_status: None,
            zoom,
        }
    }

    fn open(&mut self, path: PathBuf) {
        match Save::from_path(&path) {
            Ok(save) => {
                self.vm = ViewModel::from_save(&save);
                self.save = save;
                self.picked_path = path.clone();
                self.save_error = None;
                self.save_status = Some(format!("Loaded {}", path.display()));
            }
            Err(e) => {
                self.save_error = Some(format!("Failed to read save: {}", e));
            }
        }
    }

    fn save(&mut self, path: PathBuf) {
        // Apply all in-memory edits to the save struct first.
        self.vm.update_save(&mut self.save.save_type);

        // Serialize to bytes BEFORE touching the destination file, so a
        // serialization panic or error never truncates the user's save.
        let bytes = match self.save.write() {
            Ok(b) => b,
            Err(e) => {
                self.save_error = Some(format!("Failed to serialize save: {}", e));
                return;
            }
        };

        // Atomic write: write to `<path>.tmp`, fsync, then rename over the
        // target. The original file is never observed in a partial state.
        let mut tmp = path.clone().into_os_string();
        tmp.push(".tmp");
        let tmp_path = PathBuf::from(tmp);

        let result = (|| -> std::io::Result<()> {
            {
                let mut f = File::create(&tmp_path)?;
                f.write_all(&bytes)?;
                f.sync_all()?;
            }
            // os-level atomic replace (handles the case where `path` already exists).
            fs::rename(&tmp_path, &path)?;
            Ok(())
        })();

        match result {
            Ok(_) => {
                // Clean up a stray tmp file if rename succeeded it's already gone,
                // but if we returned early after create it might still linger.
                let _ = fs::remove_file(&tmp_path);
                self.save_error = None;
                self.save_status = Some(format!("Saved to {}", path.display()));
            }
            Err(e) => {
                let _ = fs::remove_file(&tmp_path);
                self.save_error = Some(format!("Failed to save file: {}", e));
            }
        }
    }

    fn backup_current_file(&mut self) {
        let Some(folder) = self.backup_folder.clone() else {
            self.save_error = Some("Backup folder not set.".to_string());
            return;
        };
        if !self.picked_path.exists() {
            self.save_error = Some("Backup skipped: no source file loaded.".to_string());
            return;
        }
        if let Err(e) = fs::create_dir_all(&folder) {
            self.save_error = Some(format!("Failed to create backup folder: {}", e));
            return;
        }
        let stem = self
            .picked_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("save");
        let backup_name = self
            .pending_backup_name
            .take()
            .unwrap_or_else(|| format!("{}.{}", stem, Local::now().format("%Y-%m-%d_%H-%M-%S")));
        let backup_path = folder.join(backup_name);
        match fs::copy(&self.picked_path, &backup_path) {
            Ok(_) => {
                self.save_status = Some(format!("Backup created at {}", backup_path.display()));
                self.save_error = None;
            }
            Err(e) => {
                self.save_error = Some(format!("Failed to create backup: {}", e));
            }
        }
    }

    fn perform_pending_save(&mut self) {
        let Some(path) = self.pending_save_path.take() else {
            return;
        };
        self.backup_current_file();
        if self.save_error.is_none() {
            self.save(path);
        }
        self.show_save_confirm = false;
    }

    fn load_backup_folder(cc: &eframe::CreationContext<'_>) -> Option<PathBuf> {
        cc.storage
            .and_then(|s| s.get_string("backup_folder"))
            .and_then(|s| PathBuf::from(s).canonicalize().ok())
    }

    fn load_zoom(cc: &eframe::CreationContext<'_>) -> Option<f32> {
        cc.storage
            .and_then(|s| s.get_string("zoom"))
            .and_then(|s| s.parse::<f32>().ok())
    }

    fn open_file_dialog() -> Option<PathBuf> {
        FileDialog::new()
            .add_filter("SL2/DAT", &["sl2", "dat"])
            .add_filter("SL2", &["sl2"])
            .add_filter("DAT", &["dat"])
            .add_filter("TXT", &["txt"])
            .add_filter("*", &["*"])
            .set_directory("/")
            .pick_file()
    }

    fn save_file_dialog(source: Option<&PathBuf>) -> Option<PathBuf> {
        let mut dlg = FileDialog::new()
            .add_filter("SL2/DAT", &["sl2", "dat"])
            .add_filter("SL2", &["sl2"])
            .add_filter("DAT", &["dat"])
            .add_filter("TXT", &["txt"])
            .add_filter("*", &["*"])
            .set_directory("/");
        if let Some(src) = source {
            if let Some(name) = src.file_name().and_then(|n| n.to_str()) {
                dlg = dlg.set_file_name(name);
            }
        }
        dlg.save_file()
    }
}


impl eframe::App for App {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        if let Some(folder) = &self.backup_folder {
            if let Some(path) = folder.to_str() {
                storage.set_string("backup_folder", path.to_string());
            }
        }
        storage.set_string("zoom", self.zoom.to_string());
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx();
        ctx.set_zoom_factor(self.zoom);
        // TOP PANEL
        egui::Panel::top("toolbar").default_height(35.).show(ctx, |ui| {
            ui.columns(2, |uis|{
                uis[0].with_layout(Layout::left_to_right(Align::Center),| ui| {
                    if ui.button(egui::RichText::new(format!("{} open", egui_phosphor::regular::FOLDER_OPEN))).clicked() {
                        let files = Self::open_file_dialog();
                        match files {
                            Some(path) => self.open(path),
                            None => {},
                        }
                    }
                    if ui.button(egui::RichText::new(format!("{} save", egui_phosphor::regular::FLOPPY_DISK))).clicked() {
                        if self.backup_folder.is_none() {
                            self.save_error = Some(
                                "Backup folder required. Open Settings to set one before saving."
                                    .to_string(),
                            );
                        } else {
                            let files = Self::save_file_dialog(Some(&self.picked_path));
                            if let Some(path) = files {
                                let stem = self
                                    .picked_path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("save");
                                let timestamp =
                                    Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
                                self.pending_backup_name = Some(format!("{}.{}", stem, timestamp));
                                self.pending_save_path = Some(path);
                                self.show_save_confirm = true;
                            }
                        }
                    }
                    if ui.button(egui::RichText::new(format!("{}", egui_phosphor::regular::GEAR))).clicked() {
                        self.settings_open = true;
                    }
                });
                
                uis[1].with_layout(Layout::right_to_left(egui::Align::Center),|ui| {
                    let import_button = egui::widgets::Button::new(egui::RichText::new(format!("{} Import Character", egui_phosphor::regular::DOWNLOAD_SIMPLE)));
                    if ui.add_enabled(!self.vm.steam_id.is_empty(), import_button).clicked() {
                        let files = Self::open_file_dialog();
                        match files {
                            Some(path) => {
                                match Save::from_path(&path) {
                                    Ok(save) => {
                                        self.importer_vm = ImporterViewModel::new(save, &self.vm);
                                        self.importer_open = true;
                                    },
                                    Err(_) => {},
                                }
                            },
                            None => {},
                        }
                    }
                    character_importer(ui, &mut self.importer_open, &mut self.importer_vm, &mut self.save, &mut self.vm);
                });
            });

        });

        // TOP PANEL
        egui::Panel::top("top").show(ctx, |ui| {
            if self.picked_path.exists() {
                let save_type = match self.save.save_type {
                    SaveType::Unknown => {
                        "Platform: Unknown"
                    }
                    SaveType::PC(_) => {
                        "Platform: PC"
                    }
                    SaveType::PlayStation(_) => {
                        "Platform: Playstation"
                    },
                };

                ui.columns(2,| uis| {
                    if self.vm.active.is_some_and(|valid| valid) {
                        egui::Frame::none().show(&mut uis[1], |ui| {
                            let steam_id_text_edit = egui::widgets::TextEdit::singleline(&mut self.vm.steam_id)
                            .char_limit(17)
                            .desired_width(125.);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                ui.label(format!("Character: {}", self.vm.slots[self.vm.index].general_vm.character_name));
                                
                                match self.save.save_type {
                                    SaveType::Unknown => {},
                                    SaveType::PC(_) => {
                                        let steam_id_text_edit = ui.add(steam_id_text_edit).labelled_by(ui.label("Steam Id:").id);
                                        steam_id_text_edit.on_hover_ui(|ui| {
                                            ui.label(egui::RichText::new("Important: This needs to match the id of the steam account that will use this save!").size(8.0).color(Color32::PLACEHOLDER));
                                        });
                                    },
                                    SaveType::PlayStation(_) => {},
                                };
                            });
                        });
                    }
                    egui::Frame::none().show(&mut uis[0], |ui| {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                            ui.label(format!("{}",save_type));
                        });
                    });
                });
            }
        });

        // Character List Panel
        if self.vm.active.is_some_and(|valid| valid) {
            egui::Panel::left("characters").show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .id_source("left")
                    .show(ui, |ui| {
                        ui.vertical(|ui| {
                            for i in 0..0xA {
                                if self.vm.profile_summary[i].active {
                                    let button = ui.add_sized([120., 40.], egui::Button::new(&self.vm.slots[i].general_vm.character_name));
                                    if button.clicked() {self.vm.index = i;}
                                    if self.vm.index == i {button.highlight();}
                                }
                            }
                        })
                    });
            });

            // Slot Section Panel
            egui::Panel::left("slot_sections_menu").show(ctx, |ui| {
                egui::ScrollArea::vertical() .id_source("left") .show(ui, |ui| {
                    ui.vertical(|ui| {
                        menu(ui, self);
                    })
                });
            });

            // Main Content
            egui::CentralPanel::default().show(ctx, |ui| {
                match self.current_route {
                    Route::None => none(ui),
                    Route::General => general(ui, &mut self.vm),
                    Route::Stats => stats(ui, &mut self.vm),
                    Route::Equipment => equipment(ui, &mut self.vm),
                    Route::Inventory => inventory(ui, &mut self.vm),
                    Route::EventFlags => events(ui, &mut self.vm),
                    Route::Regions => regions(ui, &mut self.vm),
                }
            });
        }
        // No file loaded View
        else {
            // Listen for dragged files and update path
            egui::CentralPanel::default().show(ctx, |ui| {
                // Check if hovering a file
                let path = ctx.input(|i| {
                    if !i.raw.hovered_files.is_empty() {
                        let file = i.raw.hovered_files[0].clone();
                        let path: std::path::PathBuf = file.path.expect("Error!");
                        return path.into_os_string().into_string().expect("");
                    }
                    "".to_string()
                }); 
                
                // Display indicator of hovering file
                ui.centered_and_justified(|ui| {
                    if !path.is_empty() {
                        let painter =
                            ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));
                
                        let screen_rect = ctx.screen_rect();
                        painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(96));
                        ui.label(egui::RichText::new(path));
                    }
                    else {
                        let style = Style::default();
                        let mut layout_job = LayoutJob::default();
                        if self.vm.active.is_some_and(|valid| !valid) {
                            RichText::new("Save file has irregular data!\n\n")
                            .color(Color32::DARK_RED)
                            .append_to(
                                &mut layout_job,
                                &style,
                                FontSelection::Default,
                                Align::Center,
                            );
                        }
                        RichText::new("Drop a save file here or click 'Open' to browse")
                        .append_to(
                            &mut layout_job,
                            &style,
                            FontSelection::Default,
                            Align::Center,
                        );
                        ui.label(layout_job);
                    }
                });

                // Check a file that has been dropped in the window
                ctx.input(|i| {
                    if !i.raw.dropped_files.is_empty() {
                        let file = i.raw.dropped_files[0].clone();
                        let path: std::path::PathBuf = file.path.expect("Error!");
                        self.open(path);
                    }
                });
            });
        }

        // Save confirmation dialog
        if self.show_save_confirm {
            let mut open = true;
            egui::Window::new("Confirm Save")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    ui.set_min_width(380.0);
                    let dest = self
                        .pending_save_path
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_default();
                    ui.label(format!("Save to: {}", dest));
                    ui.add_space(6.0);

                    match &self.backup_folder {
                        Some(folder) => {
                            let backup_name = self
                                .pending_backup_name
                                .clone()
                                .unwrap_or_else(|| "save".to_string());
                            let backup_path = folder.join(&backup_name);
                            ui.label(
                                RichText::new("A backup of the original file will be created:")
                                    .color(Color32::from_rgb(220, 200, 120)),
                            );
                            ui.label(
                                RichText::new(backup_path.display().to_string())
                                    .color(Color32::from_rgb(120, 200, 120)),
                            );
                        }
                        None => {
                            ui.label(
                                RichText::new("Backup folder not set. Open Settings to set one.")
                                    .color(Color32::DARK_RED),
                            );
                        }
                    }

                    ui.add_space(10.0);
                    let can_save = self.backup_folder.is_some();
                    ui.horizontal(|ui| {
                        if ui
                            .add_enabled(can_save, egui::Button::new("OK"))
                            .clicked()
                        {
                            self.perform_pending_save();
                        }
                        if ui.button("Cancel").clicked() {
                            self.show_save_confirm = false;
                            self.pending_save_path = None;
                            self.pending_backup_name = None;
                        }
                    });
                });
            if !open {
                self.show_save_confirm = false;
                self.pending_save_path = None;
                self.pending_backup_name = None;
            }
        }

        // Settings window
        if self.settings_open {
            let mut open = true;
            egui::Window::new("Settings")
                .open(&mut open)
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ctx, |ui| {
                    settings_view(ui, self);
                });
            if !open {
                self.settings_open = false;
            }
        }

        // Status / error toasts
        if self.save_error.is_some() || self.save_status.is_some() {
            egui::Window::new("Notice")
                .collapsible(false)
                .resizable(false)
                .anchor(egui::Align2::CENTER_TOP, [0.0, 60.0])
                .show(ctx, |ui| {
                    if let Some(err) = &self.save_error {
                        ui.label(RichText::new(err).color(Color32::DARK_RED));
                    }
                    if let Some(msg) = &self.save_status {
                        ui.label(RichText::new(msg).color(Color32::from_rgb(120, 200, 120)));
                    }
                    if ui.button("Dismiss").clicked() {
                        self.save_error = None;
                        self.save_status = None;
                    }
                });
        }
    }
}
