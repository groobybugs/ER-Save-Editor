pub mod inventory {
    use eframe::egui::{self, Color32, Ui};
    use crate::ui::inventory::{add::add, browse::browse_inventory};
    use crate::vm::{inventory::InventoryRoute, vm::vm::ViewModel};

    pub fn inventory(ui: &mut Ui, vm:&mut ViewModel) {
        // Add / Browse switch, merged into the top bar area
        // (replaces the old left menu panel to free ~140px for list + details).
        ui.add_space(6.);
        ui.columns(2, |uis| {
            let add_items = uis[0].add_sized([100., 40.], egui::Button::new("Add\n(WIP)"));
            let browse_items = uis[1].add_sized([100., 40.], egui::Button::new("Browse"));

            if add_items.clicked() {
                vm.slots[vm.index].inventory_vm.filter();
                vm.slots[vm.index].inventory_vm.current_route = InventoryRoute::Add
            }
            if browse_items.clicked() {
                vm.slots[vm.index].inventory_vm.filter();
                vm.regulation.filter(&vm.slots[vm.index].inventory_vm.current_type_route, &vm.slots[vm.index].inventory_vm.filter_text);
                vm.slots[vm.index].inventory_vm.current_route = InventoryRoute::Browse
            }

            // Highlight active
            let add_items = match vm.slots[vm.index].inventory_vm.current_route {
                InventoryRoute::Add => add_items.highlight(),
                _ => add_items,
            };
            match vm.slots[vm.index].inventory_vm.current_route {
                InventoryRoute::Browse => {browse_items.highlight();},
                _ => {},
            };

            add_items.on_hover_ui(|ui| {
                ui.label(egui::RichText::new("Warning: This is an experimental feature that is still being worked on. Use with catution.").size(8.0).color(Color32::PLACEHOLDER));
            });
        });
        ui.add_space(6.);

        match vm.slots[vm.index].inventory_vm.current_route {
            InventoryRoute::None => {ui.label("Empty");},
            InventoryRoute::Add => {add(ui, vm);},
            InventoryRoute::Browse => {browse_inventory(ui, vm);},
        }
    }
}