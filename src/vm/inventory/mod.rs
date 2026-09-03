use std::{cmp::Ordering, collections::HashMap};

use crate::{
    db::{
        accessory_name::accessory_name::ACCESSORY_NAME, aow_name::aow_name::AOW_NAME,
        armor_name::armor_name::ARMOR_NAME, gestures::GESTURES, item_name::item_name::ITEM_NAME,
        weapon_name::weapon_name::WEAPON_NAME,
    },
    save::common::save_slot::{
        EquipInventoryData, EquipInventoryItem, EquipProjectileData, GaItem, GaItemData, SaveSlot,
    },
    util::regulation::Regulation,
    vm::regulation::regulation_view_model::WepType,
};

use super::regulation::regulation_view_model::GoodsType;

#[derive(Default, Clone)]
pub enum InventoryRoute {
    #[default]
    None,
    Add,
    Browse,
}

#[derive(Default, Clone)]
pub enum InventoryTypeRoute {
    #[default]
    CommonItems,
    KeyItems,
    Weapons,
    Armors,
    AshOfWar,
    Talismans,
    Gestures,
}

#[derive(Default, PartialEq, Clone)]
pub enum InventorySubTypeRoute {
    #[default]
    None,
    WeaponLeft,
    WeaponRight,
    Head,
    Body,
    Arms,
    Legs,
    Arrow,
    Bolt,
    Talisman,
    QuickItem,
    Pouch,
}

#[derive(PartialEq, Clone, Default, Copy)]
pub enum InventoryItemType {
    #[default]
    None = -1,
    WEAPON = 0x0,
    ARMOR = 0x10000000,
    ACCESSORY = 0x20000000,
    ITEM = 0x40000000,
    GESTURE = 0x50000000,
    AOW = 0x80000000,
}
impl From<u8> for InventoryItemType {
    fn from(value: u8) -> Self {
        match value {
            0x0 => InventoryItemType::WEAPON,
            0x10 => InventoryItemType::ARMOR,
            0x20 => InventoryItemType::ACCESSORY,
            0x40 => InventoryItemType::ITEM,
            0x50 => InventoryItemType::GESTURE,
            0x80 => InventoryItemType::AOW,
            _ => InventoryItemType::None,
        }
    }
}
impl ToString for InventoryItemType {
    fn to_string(&self) -> String {
        match self {
            InventoryItemType::None => format!("None"),
            InventoryItemType::WEAPON => format!("WEAPON"),
            InventoryItemType::ARMOR => format!("ARMOR"),
            InventoryItemType::ACCESSORY => format!("ACCESSORY"),
            InventoryItemType::ITEM => format!("ITEM"),
            InventoryItemType::GESTURE => format!("GESTURE"),
            InventoryItemType::AOW => format!("AOW"),
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub enum InventoryGaitemType {
    #[default]
    EMPTY = -1,
    WEAPON = 0x80000000,
    ARMOR = 0x90000000,
    ACCESSORY = 0xa0000000,
    ITEM = 0xb0000000,
    GESTURE = 0xd0000000,
    AOW = 0xc0000000,
}
impl From<u32> for InventoryGaitemType {
    fn from(value: u32) -> Self {
        match value {
            x if x == InventoryGaitemType::WEAPON as u32 => InventoryGaitemType::WEAPON,
            x if x == InventoryGaitemType::ARMOR as u32 => InventoryGaitemType::ARMOR,
            x if x == InventoryGaitemType::ACCESSORY as u32 => InventoryGaitemType::ACCESSORY,
            x if x == InventoryGaitemType::ITEM as u32 => InventoryGaitemType::ITEM,
            x if x == InventoryGaitemType::GESTURE as u32 => InventoryGaitemType::GESTURE,
            x if x == InventoryGaitemType::AOW as u32 => InventoryGaitemType::AOW,
            _ => InventoryGaitemType::EMPTY,
        }
    }
}

/// Resolves a weapon display name from its full item id (base id + affinity offset + upgrade level).
/// Falls back to "<infusion_prefix> <base_name>" when the exact variant key is missing from
/// WEAPON_NAME, so DLC weapons whose per-affinity name entries were never added still display cleanly.
/// `base_id` is the known base weapon id when adding; pass None to derive it from `item_id`.
/// Returns None when no name can be resolved.
pub fn weapon_display_name(item_id: u32, base_id: Option<u32>) -> Option<String> {
    let variant_key = (item_id / 100) * 100;
    let upgrade_level = item_id % 100;
    let names = WEAPON_NAME.lock().unwrap();
    let resolved = match names.get(&variant_key).filter(|n| !n.is_empty()) {
        Some(name) => name.to_string(),
        None => {
            // Base weapon ids are multiples of 10000; the affinity offset is 0-1200 in steps of 100.
            let base_key = match base_id {
                Some(id) => (id / 100) * 100,
                None => (variant_key / 10000) * 10000,
            };
            let prefix = match variant_key - base_key {
                0    => None,
                100  => Some("Heavy"),
                200  => Some("Keen"),
                300  => Some("Quality"),
                400  => Some("Fire"),
                500  => Some("Flame Art"),
                600  => Some("Lightning"),
                700  => Some("Sacred"),
                800  => Some("Magic"),
                900  => Some("Cold"),
                1000 => Some("Poison"),
                1100 => Some("Blood"),
                1200 => Some("Occult"),
                _    => return None,
            };
            let base_name = names
                .get(&base_key)
                .filter(|n| !n.is_empty())
                .map(|s| s.to_string())
                .or_else(|| {
                    // Params are only present once a save's regulation has been parsed.
                    if crate::util::regulation::PARAMS
                        .read()
                        .unwrap()
                        .contains_key(&crate::util::params::params::Param::EquipParamWeapon)
                    {
                        Regulation::equip_weapon_params_map()
                            .get(&base_key)
                            .map(|p| p.name.to_string())
                    } else {
                        None
                    }
                })?;
            match prefix {
                Some(p) => {
                    if let Some(pos) = base_name.find("'s ") {
                        let (head, tail) = base_name.split_at(pos + 3);
                        format!("{}{} {}", head, p, tail.trim_start())
                    } else {
                        format!("{} {}", p, base_name)
                    }
                }
                None => base_name,
            }
        }
    };
    Some(if upgrade_level > 0 {
        format!("{} +{}", resolved, upgrade_level)
    } else {
        resolved
    })
}

#[derive(Default, Clone)]
pub struct InventoryItemViewModel {
    pub ga_item_handle: u32,
    pub item_id: u32,
    pub item_name: String,
    pub quantity: u32,
    pub inventory_index: u32,
    pub equip_index: u32,
    pub r#type: InventoryGaitemType,
}

impl InventoryItemViewModel {
    pub fn from_save(
        item_info: &EquipInventoryItem,
        equip_index: u32,
        gaitem: &GaItem,
        gaitem_type: InventoryGaitemType,
    ) -> Self {
        let gaitem_handle = item_info.ga_item_handle;
        let item_type_specific = match gaitem_type {
            InventoryGaitemType::WEAPON => {
                let id = (gaitem.item_id / 100) * 100;
                (
                    gaitem.item_id,
                    weapon_display_name(gaitem.item_id, None)
                        .unwrap_or_else(|| format!("[UNKNOWN_{}]", id)),
                )
            }
            InventoryGaitemType::ARMOR => {
                let id = gaitem.item_id ^ InventoryItemType::ARMOR as u32;
                (
                    id,
                    match ARMOR_NAME.lock().unwrap().get(&id) {
                        Some(name) => {
                            if !name.is_empty() {
                                name.to_string()
                            } else {
                                format!("[UNKNOWN_{}]", id)
                            }
                        }
                        None => format!("[UNKNOWN_{}]", id),
                    },
                )
            }
            InventoryGaitemType::ACCESSORY => {
                let id = gaitem_handle ^ InventoryGaitemType::ACCESSORY as u32;
                (
                    id,
                    match ACCESSORY_NAME.lock().unwrap().get(&id) {
                        Some(name) => {
                            if !name.is_empty() {
                                name.to_string()
                            } else {
                                format!("[UNKNOWN_{}]", id)
                            }
                        }
                        None => format!("[UNKNOWN_{}]", id),
                    },
                )
            }
            InventoryGaitemType::ITEM => {
                let id = gaitem_handle ^ InventoryGaitemType::ITEM as u32;
                (
                    id,
                    match ITEM_NAME.lock().unwrap().get(&id) {
                        Some(name) => {
                            if !name.is_empty() {
                                name.to_string()
                            } else {
                                format!("[UNKNOWN_{}]", id)
                            }
                        }
                        None => format!("[UNKNOWN_{}]", id),
                    },
                )
            }
            InventoryGaitemType::AOW => {
                let id = gaitem.item_id ^ InventoryItemType::AOW as u32;
                (
                    id,
                    match AOW_NAME.lock().unwrap().get(&id) {
                        Some(name) => {
                            if !name.is_empty() {
                                name.to_string()
                            } else {
                                format!("[UNKNOWN_{}]", id)
                            }
                        }
                        None => format!("[UNKNOWN_{}]", id),
                    },
                )
            }
            InventoryGaitemType::GESTURE => {
                (
                    gaitem.item_id,
                    match crate::db::gestures::GESTURES.iter().find(|g| g.id == gaitem.item_id as i32) {
                        Some(g) => g.name.to_string(),
                        None => format!("Gesture {}", gaitem.item_id),
                    }
                )
            }
            InventoryGaitemType::EMPTY => panic!("We shouldn't reach this!"),
        };

        Self {
            ga_item_handle: item_info.ga_item_handle,
            item_id: item_type_specific.0,
            item_name: item_type_specific.1,
            quantity: item_info.quantity,
            inventory_index: item_info.inventory_index,
            equip_index: equip_index,
            r#type: gaitem_type,
        }
    }
}

#[derive(Default, Clone)]
pub struct InventoryStorage {
    pub common_items: Vec<InventoryItemViewModel>,
    pub key_items: Vec<InventoryItemViewModel>,

    pub filtered_items: Vec<InventoryItemViewModel>,
    pub filtered_key_items: Vec<InventoryItemViewModel>,
    pub filtered_weapons: Vec<InventoryItemViewModel>,
    pub filtered_armors: Vec<InventoryItemViewModel>,
    pub filtered_aows: Vec<InventoryItemViewModel>,
    pub filtered_projectiles: Vec<InventoryItemViewModel>,
    pub filtered_accessories: Vec<InventoryItemViewModel>,

    pub common_item_count: u32,
    pub key_item_count: u32,
    pub next_acquisition_sort_order_index: u32,
    pub next_equip_index: u32,
}

#[derive(Default, Clone)]
pub struct InventoryViewModel {
    // Navigation
    pub at_storage_box: bool,
    pub at_single_items: bool,
    pub current_route: InventoryRoute,
    pub current_type_route: InventoryTypeRoute,
    pub current_bulk_type_route: InventoryTypeRoute,
    pub current_subtype_route: InventorySubTypeRoute,

    // Data
    pub filter_text: String,
    pub storage: Vec<InventoryStorage>,
    pub infusions: Vec<(i32, String)>,
    pub gaitem_map: Vec<GaItem>,
    pub projectile_list: EquipProjectileData,
    pub gaitem_data: GaItemData,
    pub bulk_items_selected: Vec<HashMap<u32, bool>>,
    pub bulk_items_max_quantity: bool,
    pub bulk_items_arrow_quantity: u32,
    pub bulk_items_weapon_level: u32,

    pub gesture_game_data: Vec<i32>,
    pub gestures: Vec<InventoryItemViewModel>,

    // Used for unqeuipping weapon or armor
    pub unarmed: InventoryItemViewModel,
    pub naked_head: InventoryItemViewModel,
    pub naked_body: InventoryItemViewModel,
    pub naked_arms: InventoryItemViewModel,
    pub naked_legs: InventoryItemViewModel,

    // Changed indicator
    pub changed: bool,

    // Log
    pub log: Vec<String>,

    // Indexes for when adding items
    next_gaitem_handle: u32,
    part_gaitem_handle: u8,
    next_aow_index: usize,
    next_armament_or_armor_index: usize,
}

impl InventoryViewModel {
    pub fn from_save(slot: &SaveSlot) -> Self {
        let mut inventory_vm = InventoryViewModel::default();
        inventory_vm.at_single_items = true;
        inventory_vm.storage = vec![InventoryStorage::default(); 2];
        inventory_vm.replace_bulk_items_selected_map(InventoryTypeRoute::CommonItems);

        // Gaitem_map
        inventory_vm.gaitem_map = slot.ga_items.clone();

        // Gaitem_data
        inventory_vm.gaitem_data = slot.ga_item_data.clone();

        // Projectile list
        inventory_vm.projectile_list = slot.equip_projectile_data.clone();

        // Gestures
        inventory_vm.gesture_game_data = slot.gesture_game_data.clone();
        for gesture_info in GESTURES {
            let is_owned = if gesture_info.index < inventory_vm.gesture_game_data.len() {
                inventory_vm.gesture_game_data[gesture_info.index] == gesture_info.id
            } else {
                false
            };

            inventory_vm.gestures.push(InventoryItemViewModel {
                ga_item_handle: 0,
                item_id: gesture_info.id as u32,
                item_name: gesture_info.name.to_string(),
                quantity: if is_owned { 1 } else { 0 },
                inventory_index: gesture_info.index as u32,
                equip_index: 0,
                r#type: InventoryGaitemType::GESTURE,
            });
        }

        // Find the next gaitem_handle used when adding new weapon, armors or ashes of war
        inventory_vm
            .gaitem_map
            .iter()
            .enumerate()
            .for_each(|(index, gaitem)| {
                if (gaitem.gaitem_handle & 0xF0000000) == InventoryGaitemType::AOW as u32 {
                    inventory_vm.next_aow_index = index;
                }
                if (gaitem.gaitem_handle & 0xFFFF) > (inventory_vm.next_gaitem_handle) {
                    inventory_vm.next_gaitem_handle = gaitem.gaitem_handle & 0xFFFF;
                    inventory_vm.next_armament_or_armor_index = index;
                }
            });
        inventory_vm.part_gaitem_handle =
            ((inventory_vm.gaitem_map[0].gaitem_handle >> 16) & 0xFF) as u8;

        inventory_vm.next_gaitem_handle = inventory_vm.next_gaitem_handle + 1;
        inventory_vm.next_aow_index = inventory_vm.next_aow_index + 1;
        inventory_vm.next_armament_or_armor_index = inventory_vm.next_armament_or_armor_index + 1;

        inventory_vm.fill_stroage_type(
            &slot.equip_inventory_data,
            slot.equip_inventory_data.next_acquisition_sort_id,
            slot.equip_inventory_data.next_equip_index,
            0,
        );
        inventory_vm.fill_stroage_type(
            &slot.storage_inventory_data,
            slot.storage_inventory_data.next_acquisition_sort_id,
            slot.storage_inventory_data.next_equip_index,
            1,
        );

        inventory_vm
    }

    fn fill_stroage_type(
        &mut self,
        equip_inventory_data: &EquipInventoryData,
        next_acquisition_sort_id: u32,
        next_equip_index: u32,
        inventory_storage_index: usize,
    ) {
        let inventory_storage = &mut self.storage[inventory_storage_index];

        for (index, item) in equip_inventory_data.common_items.iter().enumerate() {
            // Determine item type from gaitem_handle
            let inventory_gaitem_type = InventoryGaitemType::from(item.ga_item_handle & 0xf0000000);

            // Equip_index
            let equip_index = (index as u32) + 0x180;

            match inventory_gaitem_type {
                InventoryGaitemType::WEAPON => {
                    let gaitem = self
                        .gaitem_map
                        .iter()
                        .find(|gaitem| gaitem.gaitem_handle == item.ga_item_handle)
                        .unwrap();
                    let inventory_item_vm = InventoryItemViewModel::from_save(
                        &item,
                        equip_index,
                        &gaitem,
                        InventoryGaitemType::WEAPON,
                    );
                    if inventory_item_vm.item_id == 110000 && self.unarmed.item_id != 110000 {
                        self.unarmed = inventory_item_vm.clone();
                    }
                    inventory_storage
                        .common_items
                        .push(inventory_item_vm.clone());
                    inventory_storage.filtered_weapons.push(inventory_item_vm);
                }
                InventoryGaitemType::ARMOR => {
                    let gaitem = self
                        .gaitem_map
                        .iter()
                        .find(|gaitem| gaitem.gaitem_handle == item.ga_item_handle)
                        .unwrap();
                    let inventory_item_vm = InventoryItemViewModel::from_save(
                        &item,
                        equip_index,
                        &gaitem,
                        InventoryGaitemType::ARMOR,
                    );
                    if inventory_item_vm.item_id == 10000 {
                        self.naked_head = inventory_item_vm.clone();
                    } else if inventory_item_vm.item_id == 10100 {
                        self.naked_body = inventory_item_vm.clone();
                    } else if inventory_item_vm.item_id == 10200 {
                        self.naked_arms = inventory_item_vm.clone();
                    } else if inventory_item_vm.item_id == 10300 {
                        self.naked_legs = inventory_item_vm.clone();
                    }
                    inventory_storage
                        .common_items
                        .push(inventory_item_vm.clone());
                    inventory_storage.filtered_armors.push(inventory_item_vm);
                }
                InventoryGaitemType::ACCESSORY => {
                    let inventory_item_vm = InventoryItemViewModel::from_save(
                        &item,
                        equip_index,
                        &GaItem::default(),
                        InventoryGaitemType::ACCESSORY,
                    );
                    inventory_storage
                        .common_items
                        .push(inventory_item_vm.clone());
                    inventory_storage
                        .filtered_accessories
                        .push(inventory_item_vm);
                }
                InventoryGaitemType::ITEM => {
                    let inventory_item_vm = InventoryItemViewModel::from_save(
                        &item,
                        equip_index,
                        &GaItem::default(),
                        InventoryGaitemType::ITEM,
                    );
                    inventory_storage
                        .common_items
                        .push(inventory_item_vm.clone());
                    inventory_storage.filtered_items.push(inventory_item_vm);
                }
                InventoryGaitemType::AOW => {
                    let gaitem = self
                        .gaitem_map
                        .iter()
                        .find(|gaitem| gaitem.gaitem_handle == item.ga_item_handle)
                        .unwrap();
                    let inventory_item_vm = InventoryItemViewModel::from_save(
                        &item,
                        equip_index,
                        &gaitem,
                        InventoryGaitemType::AOW,
                    );
                    inventory_storage
                        .common_items
                        .push(inventory_item_vm.clone());
                    inventory_storage.filtered_aows.push(inventory_item_vm);
                }
                InventoryGaitemType::GESTURE => {}
                InventoryGaitemType::EMPTY => {
                    inventory_storage
                        .common_items
                        .push(InventoryItemViewModel::default());
                }
            }
        }

        for key_item in equip_inventory_data.key_items.iter() {
            let inventory_item_vm = InventoryItemViewModel::from_save(
                key_item,
                0,
                &GaItem::default(),
                InventoryGaitemType::ITEM,
            );
            inventory_storage.key_items.push(inventory_item_vm);
        }

        inventory_storage
            .filtered_weapons
            .sort_by(|a, b| a.item_name.cmp(&b.item_name));
        inventory_storage
            .filtered_armors
            .sort_by(|a, b| a.item_name.cmp(&b.item_name));
        inventory_storage
            .filtered_accessories
            .sort_by(|a, b| a.item_name.cmp(&b.item_name));
        inventory_storage
            .filtered_items
            .sort_by(|a, b| a.item_name.cmp(&b.item_name));
        inventory_storage
            .filtered_key_items
            .sort_by(|a, b| a.item_name.cmp(&b.item_name));
        inventory_storage
            .filtered_aows
            .sort_by(|a, b| a.item_name.cmp(&b.item_name));
        
        // No sorting for gestures as they are fixed order? Or sort?
        // Gestures are in self.gestures, not in storage.
        
        inventory_storage.common_item_count =
            equip_inventory_data.common_inventory_items_distinct_count;
        inventory_storage.key_item_count = equip_inventory_data.key_inventory_items_distinct_count;
        inventory_storage.next_acquisition_sort_order_index = next_acquisition_sort_id;
        inventory_storage.next_equip_index = next_equip_index;
    }

    pub fn filter(&mut self) {
        for inventory_storage in &mut self.storage {
            inventory_storage.filtered_weapons = inventory_storage
                .common_items
                .iter()
                .filter(|i| {
                    if i.r#type != InventoryGaitemType::WEAPON {
                        return false;
                    }
                    if self.filter_text.is_empty() {
                        return true;
                    }
                    i.item_name
                        .to_lowercase()
                        .contains(&self.filter_text.to_lowercase())
                })
                .cloned()
                .collect();

            inventory_storage.filtered_armors = inventory_storage
                .common_items
                .iter()
                .filter(|i| {
                    if i.r#type != InventoryGaitemType::ARMOR {
                        return false;
                    }
                    if self.filter_text.is_empty() {
                        return true;
                    }
                    i.item_name
                        .to_lowercase()
                        .contains(&self.filter_text.to_lowercase())
                })
                .cloned()
                .collect();

            inventory_storage.filtered_accessories = inventory_storage
                .common_items
                .iter()
                .filter(|i| {
                    if i.r#type != InventoryGaitemType::ACCESSORY {
                        return false;
                    }
                    if self.filter_text.is_empty() {
                        return true;
                    }
                    i.item_name
                        .to_lowercase()
                        .contains(&self.filter_text.to_lowercase())
                })
                .cloned()
                .collect();

            inventory_storage.filtered_items = inventory_storage
                .common_items
                .iter()
                .filter(|i| {
                    if i.r#type != InventoryGaitemType::ITEM {
                        return false;
                    }
                    if self.filter_text.is_empty() {
                        return true;
                    }
                    i.item_name
                        .to_lowercase()
                        .contains(&self.filter_text.to_lowercase())
                })
                .cloned()
                .collect();

            inventory_storage.filtered_key_items = inventory_storage
                .key_items
                .iter()
                .filter(|i| {
                    if i.quantity == 0 {
                        return false;
                    }
                    if self.filter_text.is_empty() {
                        return true;
                    }
                    i.item_name
                        .to_lowercase()
                        .contains(&self.filter_text.to_lowercase())
                })
                .cloned()
                .collect();

            inventory_storage.filtered_aows = inventory_storage
                .common_items
                .iter()
                .filter(|i| {
                    if i.r#type != InventoryGaitemType::AOW {
                        return false;
                    }
                    if self.filter_text.is_empty() {
                        return true;
                    }
                    i.item_name
                        .to_lowercase()
                        .contains(&self.filter_text.to_lowercase())
                })
                .cloned()
                .collect();

            inventory_storage.filtered_weapons.sort_by(|a, b| {
                if self.filter_text.is_empty() {
                    return a.item_name.cmp(&b.item_name);
                }
                let a_contains = a
                    .item_name
                    .to_lowercase()
                    .contains(&self.filter_text.to_lowercase());
                let b_contains = b
                    .item_name
                    .to_lowercase()
                    .contains(&self.filter_text.to_lowercase());
                if a_contains && !b_contains {
                    Ordering::Less
                } else if !a_contains && b_contains {
                    Ordering::Greater
                } else {
                    a.item_name.cmp(&b.item_name)
                }
            });

            inventory_storage.filtered_armors.sort_by(|a, b| {
                if self.filter_text.is_empty() {
                    return a.item_name.cmp(&b.item_name);
                }
                let a_contains = a
                    .item_name
                    .to_lowercase()
                    .contains(&self.filter_text.to_lowercase());
                let b_contains = b
                    .item_name
                    .to_lowercase()
                    .contains(&self.filter_text.to_lowercase());
                if a_contains && !b_contains {
                    Ordering::Less
                } else if !a_contains && b_contains {
                    Ordering::Greater
                } else {
                    a.item_name.cmp(&b.item_name)
                }
            });

            inventory_storage.filtered_accessories.sort_by(|a, b| {
                if self.filter_text.is_empty() {
                    return a.item_name.cmp(&b.item_name);
                }
                let a_contains = a
                    .item_name
                    .to_lowercase()
                    .contains(&self.filter_text.to_lowercase());
                let b_contains = b
                    .item_name
                    .to_lowercase()
                    .contains(&self.filter_text.to_lowercase());
                if a_contains && !b_contains {
                    Ordering::Less
                } else if !a_contains && b_contains {
                    Ordering::Greater
                } else {
                    a.item_name.cmp(&b.item_name)
                }
            });

            inventory_storage.filtered_items.sort_by(|a, b| {
                if self.filter_text.is_empty() {
                    return a.item_name.cmp(&b.item_name);
                }
                let a_contains = a
                    .item_name
                    .to_lowercase()
                    .contains(&self.filter_text.to_lowercase());
                let b_contains = b
                    .item_name
                    .to_lowercase()
                    .contains(&self.filter_text.to_lowercase());
                if a_contains && !b_contains {
                    Ordering::Less
                } else if !a_contains && b_contains {
                    Ordering::Greater
                } else {
                    a.item_name.cmp(&b.item_name)
                }
            });

            inventory_storage.filtered_key_items.sort_by(|a, b| {
                if self.filter_text.is_empty() {
                    return a.item_name.cmp(&b.item_name);
                }
                let a_contains = a
                    .item_name
                    .to_lowercase()
                    .contains(&self.filter_text.to_lowercase());
                let b_contains = b
                    .item_name
                    .to_lowercase()
                    .contains(&self.filter_text.to_lowercase());
                if a_contains && !b_contains {
                    Ordering::Less
                } else if !a_contains && b_contains {
                    Ordering::Greater
                } else {
                    a.item_name.cmp(&b.item_name)
                }
            });

            inventory_storage.filtered_aows.sort_by(|a, b| {
                if self.filter_text.is_empty() {
                    return a.item_name.cmp(&b.item_name);
                }
                let a_contains = a
                    .item_name
                    .to_lowercase()
                    .contains(&self.filter_text.to_lowercase());
                let b_contains = b
                    .item_name
                    .to_lowercase()
                    .contains(&self.filter_text.to_lowercase());
                if a_contains && !b_contains {
                    Ordering::Less
                } else if !a_contains && b_contains {
                    Ordering::Greater
                } else {
                    a.item_name.cmp(&b.item_name)
                }
            });
            
            // Filter gestures
            // Wait, gestures are in self.gestures, not storage.
            // But if we want to filter them in UI, we should probably have filtered_gestures?
            // Or just filter in UI. The UI code for gestures can access self.gestures.
        }
    }

    pub fn filter_with_subtype(&mut self) {
        self.filter();
        match self.current_subtype_route {
            InventorySubTypeRoute::None => {}
            InventorySubTypeRoute::WeaponLeft => {
                self.current_type_route = InventoryTypeRoute::Weapons;
                self.storage[0].filtered_weapons = self.storage[0]
                    .filtered_weapons
                    .iter()
                    .filter(|g| {
                        Regulation::equip_weapon_params_map()
                            .get(&((g.item_id / 100) * 100))
                            .is_some_and(|g| g.data.leftHandEquipable())
                    })
                    .map(|i| i.clone())
                    .collect();
            }
            InventorySubTypeRoute::WeaponRight => {
                self.current_type_route = InventoryTypeRoute::Weapons;
                self.storage[0].filtered_weapons = self.storage[0]
                    .filtered_weapons
                    .iter()
                    .filter(|g| {
                        Regulation::equip_weapon_params_map()
                            .get(&((g.item_id / 100) * 100))
                            .is_some_and(|g| g.data.rightHandEquipable())
                    })
                    .map(|i| i.clone())
                    .collect();
            }
            InventorySubTypeRoute::Head => {
                self.current_type_route = InventoryTypeRoute::Armors;
                self.storage[0].filtered_armors = self.storage[0]
                    .filtered_armors
                    .iter()
                    .filter(|g| {
                        Regulation::equip_protectors_param_map()
                            .get(&g.item_id)
                            .is_some_and(|g| g.data.equipModelCategory == 5)
                    })
                    .map(|i| i.clone())
                    .collect();
            }
            InventorySubTypeRoute::Body => {
                self.current_type_route = InventoryTypeRoute::Armors;
                self.storage[0].filtered_armors = self.storage[0]
                    .filtered_armors
                    .iter()
                    .filter(|g| {
                        Regulation::equip_protectors_param_map()
                            .get(&g.item_id)
                            .is_some_and(|g| g.data.equipModelCategory == 2)
                    })
                    .map(|i| i.clone())
                    .collect();
            }
            InventorySubTypeRoute::Arms => {
                self.current_type_route = InventoryTypeRoute::Armors;
                self.storage[0].filtered_armors = self.storage[0]
                    .filtered_armors
                    .iter()
                    .filter(|g| {
                        Regulation::equip_protectors_param_map()
                            .get(&g.item_id)
                            .is_some_and(|g| g.data.equipModelCategory == 1)
                    })
                    .map(|i| i.clone())
                    .collect();
            }
            InventorySubTypeRoute::Legs => {
                self.current_type_route = InventoryTypeRoute::Armors;
                self.storage[0].filtered_armors = self.storage[0]
                    .filtered_armors
                    .iter()
                    .filter(|g| {
                        Regulation::equip_protectors_param_map()
                            .get(&g.item_id)
                            .is_some_and(|g| g.data.equipModelCategory == 6)
                    })
                    .map(|i| i.clone())
                    .collect();
            }
            InventorySubTypeRoute::Arrow => {
                self.current_type_route = InventoryTypeRoute::Weapons;
                self.storage[0].filtered_weapons = self.storage[0]
                    .filtered_weapons
                    .iter()
                    .filter(|g| {
                        Regulation::equip_weapon_params_map()
                            .get(&g.item_id)
                            .is_some_and(|g| {
                                WepType::from(g.data.wepType) == WepType::Arrow
                                    || WepType::from(g.data.wepType) == WepType::Greatarrow
                            })
                    })
                    .map(|i| i.clone())
                    .collect();
            }
            InventorySubTypeRoute::Bolt => {
                self.current_type_route = InventoryTypeRoute::Weapons;
                self.storage[0].filtered_weapons = self.storage[0]
                    .filtered_weapons
                    .iter()
                    .filter(|g| {
                        Regulation::equip_weapon_params_map()
                            .get(&g.item_id)
                            .is_some_and(|g| {
                                WepType::from(g.data.wepType) == WepType::Bolt
                                    || WepType::from(g.data.wepType) == WepType::BallistaBolt
                            })
                    })
                    .map(|i| i.clone())
                    .collect();
            }
            InventorySubTypeRoute::Talisman => {
                self.current_type_route = InventoryTypeRoute::Talismans;
            }
            InventorySubTypeRoute::QuickItem | InventorySubTypeRoute::Pouch => {
                self.current_type_route = InventoryTypeRoute::CommonItems;
                self.storage[0].filtered_items = self.storage[0]
                    .filtered_items
                    .iter()
                    .filter(|g| {
                        Regulation::equip_goods_param_map()
                            .get(&g.item_id)
                            .is_some_and(|g| {
                                GoodsType::from(g.data.goodsType) == GoodsType::NormalItem
                            })
                    })
                    .map(|i| i.clone())
                    .collect();
            }
        }
    }
}

// Splitting up inventory into multiple files
mod add_bulk;
mod add_single;

#[cfg(test)]
mod tests {
    use super::weapon_display_name;

    #[test]
    fn exact_variant_name_is_used() {
        // Heavy Backhand Blade has its own entry in WEAPON_NAME.
        assert_eq!(
            weapon_display_name(64500100, Some(64500000)),
            Some("Heavy Backhand Blade".to_string())
        );
    }

    #[test]
    fn missing_variant_falls_back_to_prefixed_base_name() {
        // 3520200 (Keen Lizard Greatsword) has no variant entry, only base 3520000.
        assert_eq!(
            weapon_display_name(3520200, Some(3520000)),
            Some("Keen Lizard Greatsword".to_string())
        );
        // Same resolution when the base id has to be derived from the item id (browse path).
        assert_eq!(
            weapon_display_name(3520200, None),
            Some("Keen Lizard Greatsword".to_string())
        );
    }

    #[test]
    fn upgrade_level_is_appended() {
        assert_eq!(
            weapon_display_name(3520225, None),
            Some("Keen Lizard Greatsword +25".to_string())
        );
    }

    #[test]
    fn unresolvable_ids_return_none() {
        // Offset 1300 is not a valid affinity.
        assert_eq!(weapon_display_name(3521300, None), None);
        // Unknown base weapon id (regulation params not loaded in tests).
        assert_eq!(weapon_display_name(999990100, None), None);
    }
}
