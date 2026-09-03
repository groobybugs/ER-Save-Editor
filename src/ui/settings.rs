pub mod settings {
    use crate::App;
    use eframe::egui::{self, Color32, RichText, Ui};
    use rfd::FileDialog;

    pub fn settings(ui: &mut Ui, app: &mut App) {
        ui.heading("Settings");
        ui.add_space(10.0);

        ui.group(|ui| {
            ui.label(RichText::new("Auto-backup").strong());
            ui.add_space(4.0);
            ui.label(
                RichText::new("Auto-backup is enabled for your own safety.")
                    .color(Color32::from_rgb(120, 200, 120)),
            );

            ui.add_space(8.0);
            ui.label(RichText::new("Backup folder:").strong());

            ui.horizontal(|ui| {
                if ui.button("Select folder...").clicked() {
                    if let Some(folder) = FileDialog::new().pick_folder() {
                        app.backup_folder = Some(folder);
                    }
                }
                if ui
                    .add_enabled(app.backup_folder.is_some(), egui::Button::new("Clear"))
                    .clicked()
                {
                    app.backup_folder = None;
                }
            });

            match &app.backup_folder {
                Some(path) => {
                    ui.label(
                        RichText::new(path.display().to_string())
                            .color(Color32::from_rgb(120, 200, 120)),
                    );
                }
                None => {
                    ui.label(
                        RichText::new("No folder set — saving is blocked until one is selected")
                            .color(Color32::DARK_RED),
                    );
                }
            }

            ui.add_space(4.0);
            ui.label(
                RichText::new(
                    "Backups are saved as <filename sl2 or dat>.YYYY-MM-DD_HH-MM-SS in the chosen folder.",
                )
                .size(10.0)
                .color(Color32::PLACEHOLDER),
            );
        });

        ui.add_space(10.0);
        ui.group(|ui| {
            ui.label(RichText::new("UI Zoom").strong());
            ui.add_space(4.0);
            // The slider only edits a pending value. Applying the zoom live
            // rescales this slider mid-drag, so the pointer-to-value mapping
            // jumps and the drag flings to min/max. Apply on release instead.
            let pending = ui
                .horizontal(|ui| {
                    ui.label("Zoom:");
                    ui.add(
                        egui::Slider::new(&mut app.zoom, 0.5..=2.5)
                            .step_by(0.05)
                            .fixed_decimals(2)
                            .text("x"),
                    )
                })
                .inner;

            if pending.drag_stopped()
                || (pending.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)))
            {
                app.zoom = app.zoom.clamp(0.5, 2.5);
                app.zoom_applied = app.zoom;
                ui.ctx().set_zoom_factor(app.zoom_applied);
            }

            ui.add_space(4.0);
            ui.horizontal(|ui| {
                if ui.button("Apply").clicked() {
                    app.zoom = app.zoom.clamp(0.5, 2.5);
                    app.zoom_applied = app.zoom;
                    ui.ctx().set_zoom_factor(app.zoom_applied);
                }
                if ui.button("Reset 1.0x").clicked() {
                    app.zoom = 1.0;
                    app.zoom_applied = 1.0;
                    ui.ctx().set_zoom_factor(1.0);
                }
                for preset in [0.75, 1.0, 1.5, 2.0] {
                    if ui.button(format!("{preset:.2}x")).clicked() {
                        app.zoom = preset;
                        app.zoom_applied = preset;
                        ui.ctx().set_zoom_factor(preset);
                    }
                }
            });

            ui.add_space(4.0);
            if (app.zoom - app.zoom_applied).abs() > f32::EPSILON {
                ui.label(RichText::new(format!(
                    "Pending {:.2}x — release the slider or press Apply (applied {:.2}x).",
                    app.zoom, app.zoom_applied
                )));
            } else {
                ui.label(format!("Applied {:.2}x.", app.zoom_applied));
            }
            ui.label(
                RichText::new(
                    "Adjust magnification of the whole UI. 1.0x = native. Default 1.0x.",
                )
                .size(10.0)
                .color(Color32::PLACEHOLDER),
            );
        });
    }
}
