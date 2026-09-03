pub mod stats {
    use std::ops::RangeInclusive;

    use eframe::egui::{self, Ui};
    use egui_extras::{Column, TableBody, TableBuilder};
    use crate::{db::classes::classes::{ArcheType, STARTER_CLASSES}, vm::vm::vm::ViewModel};

    pub fn stats(ui: &mut Ui,  vm: &mut ViewModel) {
        egui::Frame::default()
        .show(ui, |ui| {
            ui.with_layout( egui::Layout::top_down_justified(egui::Align::Min),|ui|{
                ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui|{

                    let stats_vm = &mut vm.slots[vm.index].stats_vm;

                    let class_options = {
                        let classes = STARTER_CLASSES.lock().unwrap();
                        let mut keys: Vec<ArcheType> = classes.keys().copied().collect();
                        keys.sort();
                        keys
                    };

                    let mut selected_class = stats_vm.arche_type;
                    ui.horizontal(|ui| {
                        ui.label("Class:");
                        egui::ComboBox::from_id_source(("stats_class_selector", vm.index))
                            .selected_text(selected_class.to_string())
                            .show_ui(ui, |ui| {
                                for class in &class_options {
                                    ui.selectable_value(&mut selected_class, *class, class.to_string());
                                }
                            });
                    });

                    if selected_class != stats_vm.arche_type {
                        stats_vm.arche_type = selected_class;
                        let base_stats = {
                            let classes = STARTER_CLASSES.lock().unwrap();
                            *classes
                                .get(&selected_class)
                                .expect("Starter class definition missing")
                        };
                        stats_vm.vigor = stats_vm.vigor.max(base_stats.vigor);
                        stats_vm.mind = stats_vm.mind.max(base_stats.mind);
                        stats_vm.endurance = stats_vm.endurance.max(base_stats.endurance);
                        stats_vm.strength = stats_vm.strength.max(base_stats.strength);
                        stats_vm.dexterity = stats_vm.dexterity.max(base_stats.dexterity);
                        stats_vm.intelligence =
                            stats_vm.intelligence.max(base_stats.intelligence);
                        stats_vm.faith = stats_vm.faith.max(base_stats.faith);
                        stats_vm.arcane = stats_vm.arcane.max(base_stats.arcane);
                    }

                    ui.add_space(8.0);

                    let class = {
                        let classes = STARTER_CLASSES.lock().unwrap();
                        *classes
                            .get(&stats_vm.arche_type)
                            .expect("Starter class definition missing")
                    };

                    // Calculate level from stats
                    let level =
                        stats_vm.vigor +
                        stats_vm.mind +
                        stats_vm.endurance +
                        stats_vm.strength +
                        stats_vm.dexterity +
                        stats_vm.intelligence +
                        stats_vm.faith +
                        stats_vm.arcane -
                        79;


                    let table = TableBuilder::new(ui)
                    .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                    .column(Column::initial(100.0))
                    .column(Column::initial(100.0));

                    table.body(|mut body| {

                        // Level
                        body.row(24., |mut row| {
                            row.col(|ui| { 
                                ui.label("Level:"); }); 
                            row.col(|ui| { 
                                ui.label(format!("{:6}", level));
                            });
                        });
                        
                        // Stats
                        self::stat_field(&mut body, class.vigor..=99, "Vigor:", &mut stats_vm.vigor);
                        self::stat_field(&mut body, class.mind..=99, "Mind:", &mut stats_vm.mind);
                        self::stat_field(&mut body, class.endurance..=99, "Endurance:", &mut stats_vm.endurance);
                        self::stat_field(&mut body, class.strength..=99, "Strength:", &mut stats_vm.strength);
                        self::stat_field(&mut body, class.dexterity..=99, "Dexterity:", &mut stats_vm.dexterity);
                        self::stat_field(&mut body, class.intelligence..=99, "Intelligence:", &mut stats_vm.intelligence);
                        self::stat_field(&mut body, class.faith..=99, "Faith:", &mut stats_vm.faith);
                        self::stat_field(&mut body, class.arcane..=99, "Arcane:", &mut stats_vm.arcane);

                        // DLC Stats
                        self::space(&mut body, 8.);
                        self::stat_field(&mut body, 0..=20, "Scadutree Blessing:", &mut stats_vm.scadutree);
                        self::stat_field(&mut body, 0..=10, "Shadow Realm Blessing:", &mut stats_vm.spirit_ash);

                        // Space
                        self::space(&mut body, 8.);

                        // Souls
                        let field = egui::widgets::DragValue::new(&mut stats_vm.souls)
                            .clamp_range(0..=999999999)
                            .custom_formatter(|n, _|{
                                format!("{:09}", n)
                            });
                        body.row(24., |mut row| {
                            row.col(|ui| {
                                ui.label("Current Souls:");
                            });
                            row.col(|ui| {
                                ui.add(field);
                            });
                        });
                    });
                });
            })
        });
    }

    fn stat_field(body: &mut TableBody, range: RangeInclusive<u32>, name: &str, value: &mut u32) {
        let field = egui::widgets::DragValue::new(value).clamp_range(range);
        body.row(24., |mut row| {
            row.col(|ui| {
                ui.label(name);
            });
            row.col(|ui| {
                ui.add(field);
            });
        });
    }

    fn space(body: &mut TableBody, height: f32) {
        body.row(height, |mut row| {
            row.col(|_| {
            });
            row.col(|_| {
            });
        });
    }
}
