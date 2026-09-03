pub mod save {
    use std::{fs, io, path::PathBuf};
    use binary_reader::BinaryReader;
    use crate::{
        read::read::Read, save::{
            common::{ save_slot::{EquipInventoryData, EquipProjectileData, GaItem, GaItemData, SaveSlot}, user_data_10::ProfileSummary, user_data_11::UserData11 },
            pc::pc_save::PCSave, 
            playstation::ps_save::PSSave, 
        }, util::{bit::bit::set_bit, regulation::Regulation}, write::write::Write
    };

    // Using a checksum of the regulation bin file to check for Save Wizard .txt save file
    // const REGULATION_MD5_CHECKSUM: [u8; 0x10] =[0x2E, 0x88, 0x1A, 0x15, 0xAC, 0x05, 0x88, 0x8D, 0xF2, 0xC2, 0x6A, 0xEC, 0xC2, 0x90, 0x89, 0x23];

    pub enum SaveType {
        Unknown,
        PC(PCSave),
        PlayStation(PSSave)
    }
    
    #[allow(unused)]
    impl SaveType {
        pub fn get_global_steam_id(&self) -> u64 {
            match self {
                SaveType::Unknown => todo!(),
                SaveType::PC(pc_save) => {
                    pc_save.user_data_10.steam_id
                }
                SaveType::PlayStation(_) => 0,
            }
        }

        pub fn set_global_steam_id(&mut self, steam_id: u64) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.user_data_10.steam_id = steam_id;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.user_data_10.steam_id = 0;
                },
            }
        }

        pub fn active_slots(&self) -> [bool; 10] {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => pc_save.user_data_10.active_slot,
                SaveType::PlayStation(ps_save) => ps_save.user_data_10.active_slot,
            }
        }

        pub fn get_character_steam_id(&self, index: usize) -> u64 {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.steam_id
                }
                SaveType::PlayStation(_) => 0,
            }
        }

        pub fn set_character_steam_id(&mut self, index: usize, steam_id: u64) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.steam_id = steam_id;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].steam_id = 0;
                },
            }
        }

        pub fn set_character_name(&mut self, index: usize, character_name_str: String) {
            let mut character_name: [u16; 0x10] = [0; 0x10];
            let mut character_name2: [u16; 0x11] = [0; 0x11];
            for (i, char) in character_name_str.chars().enumerate() {
                character_name[i] = char as u16;
                character_name2[i] = char as u16;
            }
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.character_name.copy_from_slice(&character_name);
                    pc_save.user_data_10.profile_summary[index].character_name.copy_from_slice(&character_name2);
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.character_name.copy_from_slice(&character_name);
                    ps_save.user_data_10.profile_summary[index].character_name.copy_from_slice(&character_name2);
                },
            }
        }

        pub fn set_character_gender(&mut self, index: usize, gender: u8) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.gender = gender;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.gender = gender;
                },
            }
        }

        pub fn set_character_arche_type(&mut self, index: usize, arche_type: u8) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.arche_type = arche_type;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.arche_type = arche_type;
                },
            }
        }

        pub fn set_character_health(&mut self, index: usize, health: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.health = health;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.health = health;
                },
            }
        }

        pub fn set_character_base_max_health(&mut self, index: usize, base_max_health: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.base_max_health = base_max_health;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.base_max_health = base_max_health;
                },
            }
        }

        pub fn set_character_fp(&mut self, index: usize, fp: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.fp = fp;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.fp = fp;
                },
            }
        }

        pub fn set_character_base_max_fp(&mut self, index: usize, base_max_fp: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.base_max_fp = base_max_fp;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.base_max_fp = base_max_fp;
                },
            }
        }

        pub fn set_character_sp(&mut self, index: usize, sp: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.sp = sp;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.sp = sp;
                },
            }
        }

        pub fn set_character_base_max_sp(&mut self, index: usize, base_max_sp: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.base_max_sp = base_max_sp;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.base_max_sp = base_max_sp;
                },
            }
        }
        
        pub fn set_character_level(&mut self, index: usize, level: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.level = level;
                    pc_save.user_data_10.profile_summary[index].level = level;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.level = level;
                },
            }
        }

        pub fn set_character_vigor(&mut self, index: usize, vigor: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.vigor = vigor;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.vigor = vigor;
                },
            }
        }

        pub fn set_character_mind(&mut self, index: usize, mind: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.mind = mind;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.mind = mind;
                },
            }
        }

        pub fn set_character_endurance(&mut self, index: usize, endurance: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.endurance = endurance;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.endurance = endurance;
                },
            }
        }

        pub fn set_character_strength(&mut self, index: usize, strength: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.strength = strength;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.strength = strength;
                },
            }
        }

        pub fn set_character_dexterity(&mut self, index: usize, dexterity: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.dexterity = dexterity;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.dexterity = dexterity;
                },
            }
        }

        pub fn set_character_intelligence(&mut self, index: usize, intelligence: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.intelligence = intelligence;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.intelligence = intelligence;
                },
            }
        }

        pub fn set_character_faith(&mut self, index: usize, faith: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.faith = faith;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.faith = faith;
                },
            }
        }

        pub fn set_character_arcane(&mut self, index: usize, arcane: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.arcane = arcane;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.arcane = arcane;
                },
            }
        }

        pub fn set_character_souls(&mut self, index: usize, souls: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let orgininal_soulsmemory = pc_save.save_slots[index].save_slot.player_game_data.soulsmemory;
                    let orgininal_souls = pc_save.save_slots[index].save_slot.player_game_data.soulsmemory;
                    pc_save.save_slots[index].save_slot.player_game_data.souls = souls;
                    if souls > orgininal_souls {
                        pc_save.save_slots[index].save_slot.player_game_data.soulsmemory = orgininal_soulsmemory + souls;
                    }
                }
                SaveType::PlayStation(ps_save) => {
                    let orgininal_soulsmemory = ps_save.save_slots[index].player_game_data.soulsmemory;
                    let orgininal_souls = ps_save.save_slots[index].player_game_data.soulsmemory;
                    ps_save.save_slots[index].player_game_data.souls = souls;
                    if souls > orgininal_souls {
                        ps_save.save_slots[index].player_game_data.soulsmemory = orgininal_soulsmemory + souls;
                    }
                },
            }
        }

        pub fn set_character_event_flag(&mut self, index: usize, offset: usize, bit_pos: u8, state: bool) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let event_byte = pc_save.save_slots[index].save_slot.event_flags.flags[offset];
                    pc_save.save_slots[index].save_slot.event_flags.flags[offset] = set_bit(event_byte, bit_pos, state);
                }
                SaveType::PlayStation(ps_save) => {
                    let event_byte = ps_save.save_slots[index].event_flags.flags[offset];
                    ps_save.save_slots[index].event_flags.flags[offset] = set_bit(event_byte, bit_pos, state);
                },
            }
        }

        pub fn add_region(&mut self, index: usize, region_id: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let res = pc_save.save_slots[index].save_slot.regions.unlocked_regions.iter().position(|r| *r == region_id);
                    match res {
                        Some(i) => {},
                        None => {
                            pc_save.save_slots[index].save_slot.regions.unlocked_regions.push(region_id);
                            pc_save.save_slots[index].save_slot.regions.unlocked_regions_count = pc_save.save_slots[index].save_slot.regions.unlocked_regions_count + 1;
                        },
                    }
                }
                SaveType::PlayStation(ps_save) => {
                    let res = ps_save.save_slots[index].regions.unlocked_regions.iter().position(|r| *r == region_id);
                    match res {
                        Some(i) => {},
                        None => {
                            ps_save.save_slots[index].regions.unlocked_regions.push(region_id);
                            ps_save.save_slots[index].regions.unlocked_regions_count = ps_save.save_slots[index].regions.unlocked_regions_count + 1;
                        },
                    }
                },
            }
        }

        pub fn remove_region(&mut self, index: usize, region_id: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let res = pc_save.save_slots[index].save_slot.regions.unlocked_regions.iter().position(|r| *r == region_id);
                    match res {
                        Some(i) => {
                            pc_save.save_slots[index].save_slot.regions.unlocked_regions.swap_remove(i);
                            pc_save.save_slots[index].save_slot.regions.unlocked_regions_count = pc_save.save_slots[index].save_slot.regions.unlocked_regions_count - 1;
                        },
                        None => {},
                    }
                }
                SaveType::PlayStation(ps_save) => {
                    let res = ps_save.save_slots[index].regions.unlocked_regions.iter().position(|r| *r == region_id);
                    match res {
                        Some(i) => {
                            ps_save.save_slots[index].regions.unlocked_regions.swap_remove(i);
                            ps_save.save_slots[index].regions.unlocked_regions_count = ps_save.save_slots[index].regions.unlocked_regions_count - 1;
                        },
                        None => {},
                    }
                },
            }
        }

        pub fn get_profile_summary(&self, index: usize) -> ProfileSummary {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => pc_save.user_data_10.profile_summary[index],
                SaveType::PlayStation(ps_save) => ps_save.user_data_10.profile_summary[index],
            }
        }

        pub fn set_profile_summary(&mut self, index:usize, profile_summary: ProfileSummary) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {pc_save.user_data_10.profile_summary[index] = profile_summary},
                SaveType::PlayStation(ps_save) => {ps_save.user_data_10.profile_summary[index] = profile_summary},
            }
        }

        pub fn get_slot(&self, index: usize) -> &SaveSlot {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => &pc_save.save_slots[index].save_slot,
                SaveType::PlayStation(ps_save) => &ps_save.save_slots[index],
            }
        }

        pub fn set_slot(&mut self, index:usize, save_slot: &SaveSlot) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {pc_save.save_slots[index].save_slot = save_slot.clone()},
                SaveType::PlayStation(ps_save) => {ps_save.save_slots[index] = save_slot.clone()},
            }
        }
        
        pub fn get_user_data_11(&mut self) -> &UserData11{
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => &pc_save.user_data_11.user_data_11,
                SaveType::PlayStation(ps_save) => &ps_save.user_data_11,
            }
        }

        pub fn get_regulation(&self) -> &[u8]{
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => &pc_save.user_data_11.user_data_11.regulation,
                SaveType::PlayStation(ps_save) => &ps_save.user_data_11.regulation,
            }
        }

        pub fn set_gaitem_map(&mut self, index: usize, ga_items: Vec<GaItem>) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {pc_save.save_slots[index].save_slot.ga_items = ga_items;}
                SaveType::PlayStation(ps_save) => {ps_save.save_slots[index].ga_items = ga_items;}
            }
        }

        pub fn set_held_inventory(&mut self, index: usize, held_inventory: EquipInventoryData) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.equip_inventory_data = held_inventory;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].equip_inventory_data = held_inventory;
                }
            }
        }

        pub fn set_storage_box_inventory(&mut self, index: usize, storage_box_inventory: EquipInventoryData) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.storage_inventory_data = storage_box_inventory;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].storage_inventory_data = storage_box_inventory;
                }
            }
        }

        pub fn set_gaitem_item_data(&mut self, index: usize, gaitem_data: GaItemData) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.ga_item_data = gaitem_data;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].ga_item_data = gaitem_data;
                }
            }
        }

        pub fn set_gesture_game_data(&mut self, index: usize, gesture_game_data: Vec<i32>) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.gesture_game_data = gesture_game_data;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].gesture_game_data = gesture_game_data;
                }
            }
        }

        pub fn set_quickslot_item(&mut self, slot_index: usize, quickslot_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[slot_index].save_slot.equipped_items.quickitems[quickslot_index] = item_id;
                    pc_save.save_slots[slot_index].save_slot.equip_item_data.quick_slot_items[quickslot_index].item_id = gaitem_handle;
                    pc_save.save_slots[slot_index].save_slot.equip_item_data.quick_slot_items[quickslot_index].equipment_index = equip_index;
                },
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[slot_index].equipped_items.quickitems[quickslot_index] = item_id;
                    ps_save.save_slots[slot_index].equip_item_data.quick_slot_items[quickslot_index].item_id = gaitem_handle;
                    ps_save.save_slots[slot_index].equip_item_data.quick_slot_items[quickslot_index].equipment_index = equip_index;
                },
            }
        }

        pub fn set_pouch_item(&mut self, slot_index: usize, pouch_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[slot_index].save_slot.equipped_items.pouch[pouch_index] = item_id;
                    pc_save.save_slots[slot_index].save_slot.equip_item_data.pouch_items[pouch_index].item_id = gaitem_handle;
                    pc_save.save_slots[slot_index].save_slot.equip_item_data.pouch_items[pouch_index].equipment_index = equip_index;
                },
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[slot_index].equipped_items.pouch[pouch_index] = item_id;
                    ps_save.save_slots[slot_index].equip_item_data.pouch_items[pouch_index].item_id = gaitem_handle;
                    ps_save.save_slots[slot_index].equip_item_data.pouch_items[pouch_index].equipment_index = equip_index;
                },
            }
        }

        pub fn set_left_weapon_slot(&mut self, slot_index: usize, weapon_slot_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.left_hand_armaments[weapon_slot_index] = equip_index;
                    slot.chr_asm.left_hand_armaments[weapon_slot_index] = item_id;
                    slot.chr_asm2.left_hand_armaments[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.left_hand_armaments[weapon_slot_index] = item_id;
                    profile_summary.equipment_gaitem.left_hand_armaments[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.left_hand_armaments[weapon_slot_index] = item_id;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.left_hand_armaments[weapon_slot_index] = equip_index;
                    slot.chr_asm.left_hand_armaments[weapon_slot_index] = item_id;
                    slot.chr_asm2.left_hand_armaments[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.left_hand_armaments[weapon_slot_index] = item_id;
                    profile_summary.equipment_gaitem.left_hand_armaments[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.left_hand_armaments[weapon_slot_index] = item_id;
                },
            }
        }
        
        pub fn set_right_weapon_slot(&mut self, slot_index: usize, weapon_slot_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.right_hand_armaments[weapon_slot_index] = equip_index;
                    slot.chr_asm.right_hand_armaments[weapon_slot_index] = item_id;
                    slot.chr_asm2.right_hand_armaments[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.right_hand_armaments[weapon_slot_index] = item_id;
                    profile_summary.equipment_gaitem.right_hand_armaments[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.right_hand_armaments[weapon_slot_index] = item_id;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.right_hand_armaments[weapon_slot_index] = equip_index;
                    slot.chr_asm.right_hand_armaments[weapon_slot_index] = item_id;
                    slot.chr_asm2.right_hand_armaments[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.right_hand_armaments[weapon_slot_index] = item_id;
                    profile_summary.equipment_gaitem.right_hand_armaments[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.right_hand_armaments[weapon_slot_index] = item_id;
                },
            }
        }
        
        pub fn set_arrow_slot(&mut self, slot_index: usize, weapon_slot_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.arrows[weapon_slot_index] = equip_index;
                    slot.chr_asm.arrows[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    slot.chr_asm2.arrows[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.arrows[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    profile_summary.equipment_gaitem.arrows[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.arrows[weapon_slot_index] = item_id;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.arrows[weapon_slot_index] = equip_index;
                    slot.chr_asm.arrows[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    slot.chr_asm2.arrows[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.arrows[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    profile_summary.equipment_gaitem.arrows[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.arrows[weapon_slot_index] = item_id;
                },
            }
        }
        
        pub fn set_bolt_slot(&mut self, slot_index: usize, weapon_slot_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.bolts[weapon_slot_index] = equip_index;
                    slot.chr_asm.bolts[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    slot.chr_asm2.bolts[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.bolts[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    profile_summary.equipment_gaitem.bolts[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.bolts[weapon_slot_index] = item_id;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.bolts[weapon_slot_index] = equip_index;
                    slot.chr_asm.bolts[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    slot.chr_asm2.bolts[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.bolts[weapon_slot_index] = if item_id == 0 {u32::MAX} else {item_id};
                    profile_summary.equipment_gaitem.bolts[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.bolts[weapon_slot_index] = item_id;
                },
            }
        }
        
        pub fn set_talisman_slot(&mut self, slot_index: usize, weapon_slot_index: usize,  gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.talismans[weapon_slot_index] = equip_index;
                    slot.chr_asm.talismans[weapon_slot_index] = item_id;
                    slot.chr_asm2.talismans[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.talismans[weapon_slot_index] = item_id | 0x20000000;
                    profile_summary.equipment_gaitem.talismans[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.talismans[weapon_slot_index] = item_id | 0x20000000;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.talismans[weapon_slot_index] = equip_index;
                    slot.chr_asm.talismans[weapon_slot_index] = item_id;
                    slot.chr_asm2.talismans[weapon_slot_index] = gaitem_handle;
                    slot.equipped_items.talismans[weapon_slot_index] = item_id | 0x20000000;
                    profile_summary.equipment_gaitem.talismans[weapon_slot_index] = gaitem_handle;
                    profile_summary.equipment_item.talismans[weapon_slot_index] = item_id | 0x20000000;
                },
            }
        }
        
        pub fn set_head_gear(&mut self, slot_index: usize, gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.head = equip_index;
                    slot.chr_asm.head = item_id;
                    slot.chr_asm2.head = gaitem_handle;
                    slot.equipped_items.head = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.head = gaitem_handle;
                    profile_summary.equipment_item.head = item_id | 0x10000000;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.head = equip_index;
                    slot.chr_asm.head = item_id;
                    slot.chr_asm2.head = gaitem_handle;
                    slot.equipped_items.head = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.head = gaitem_handle;
                    profile_summary.equipment_item.head = item_id | 0x10000000;
                },
            }
        }
        
        pub fn set_chest_piece(&mut self, slot_index: usize, gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.chest = equip_index;
                    slot.chr_asm.chest = item_id;
                    slot.chr_asm2.chest = gaitem_handle;
                    slot.equipped_items.chest = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.chest = gaitem_handle;
                    profile_summary.equipment_item.chest = item_id | 0x10000000;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.chest = equip_index;
                    slot.chr_asm.chest = item_id;
                    slot.chr_asm2.chest = gaitem_handle;
                    slot.equipped_items.chest = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.chest = gaitem_handle;
                    profile_summary.equipment_item.chest = item_id | 0x10000000;
                },
            }
        }
        
        pub fn set_gauntlets(&mut self, slot_index: usize, gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.arms = equip_index;
                    slot.chr_asm.arms = item_id;
                    slot.chr_asm2.arms = gaitem_handle;
                    slot.equipped_items.arms = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.arms = gaitem_handle;
                    profile_summary.equipment_item.arms = item_id | 0x10000000;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.arms = equip_index;
                    slot.chr_asm.arms = item_id;
                    slot.chr_asm2.arms = gaitem_handle;
                    slot.equipped_items.arms = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.arms = gaitem_handle;
                    profile_summary.equipment_item.arms = item_id | 0x10000000;
                },
            }
        }
        
        pub fn set_leggings(&mut self, slot_index: usize, gaitem_handle: u32, item_id: u32, equip_index: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    let slot = &mut pc_save.save_slots[slot_index].save_slot;
                    let profile_summary = &mut pc_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.legs = equip_index;
                    slot.chr_asm.legs = item_id;
                    slot.chr_asm2.legs = gaitem_handle;
                    slot.equipped_items.legs = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.legs = gaitem_handle;
                    profile_summary.equipment_item.legs = item_id | 0x10000000;
                },
                SaveType::PlayStation(ps_save) => {
                    let slot = &mut ps_save.save_slots[slot_index];
                    let profile_summary = &mut ps_save.user_data_10.profile_summary[slot_index];
                    slot.equip_data.legs = equip_index;
                    slot.chr_asm.legs = item_id;
                    slot.chr_asm2.legs = gaitem_handle;
                    slot.equipped_items.legs = item_id | 0x10000000;
                    profile_summary.equipment_gaitem.legs = gaitem_handle;
                    profile_summary.equipment_item.legs = item_id | 0x10000000;
                },
            }
        }

        pub fn set_equip_projectile_data(&mut self, index: usize, projectile_list: EquipProjectileData) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.equip_projectile_data = projectile_list.clone();
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].equip_projectile_data = projectile_list.clone();
                }
            }
        }

        pub fn set_match_making_wpn_lvl(&mut self, index: usize, weapon_level: u8) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index].save_slot.player_game_data.match_making_wpn_lvl = weapon_level;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.match_making_wpn_lvl = weapon_level;
                }
            }
        }

        // DLC
        pub fn set_character_scadutree_lvl(&mut self, index: usize, scadutree_lvl: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index]
                        .save_slot
                        .player_game_data
                        .scadutree_lvl = scadutree_lvl as u8;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.scadutree_lvl = scadutree_lvl as u8;
                }
            }
        }

        pub fn set_character_spirit_ash_lvl(&mut self, index: usize, spirit_ash_lvl: u32) {
            match self {
                SaveType::Unknown => panic!("Why are we here?"),
                SaveType::PC(pc_save) => {
                    pc_save.save_slots[index]
                        .save_slot
                        .player_game_data
                        .spirit_ash_lvl = spirit_ash_lvl as u8;
                }
                SaveType::PlayStation(ps_save) => {
                    ps_save.save_slots[index].player_game_data.spirit_ash_lvl =
                        spirit_ash_lvl as u8;
                }
            }
        }
    }

    pub struct Save {
        pub save_type: SaveType,
    }

    impl Default for Save {
        fn default() -> Self {
            Self {  
                save_type: SaveType::Unknown,
            }
        }
    }
    
    impl Read for Save {
        fn read(br: &mut BinaryReader) -> Result<Self, io::Error> {
            let mut save = Save::default();

            if Self::is_pc(br) {
                save.save_type = SaveType::PC(PCSave::read(br)?);
            }
            else if Self::is_ps_save_wizard(br) {
                save.save_type = SaveType::PlayStation(PSSave::read(br)?);
            }
            else {
                return Err( std::io::Error::new(io::ErrorKind::InvalidData, "Invalid data!") );
            }
            
            Ok(save)
        }
    }

    impl Write for Save {
        fn write(&self) -> Result<Vec<u8>, io::Error> {
            let save_bytes: Vec<u8> =  match &self.save_type {
                SaveType::Unknown => Vec::new(),
                SaveType::PC(pc_save) => pc_save.write()?,
                SaveType::PlayStation(ps_save) => ps_save.write()?,
            };
            Ok(save_bytes)
        }
    }

    impl Save {
        pub fn from_path(path: &PathBuf) -> Result<Save, io::Error> {
            let contents = fs::read(path).expect("Should have been able to read the file");
            let mut br = BinaryReader::from_u8(&contents);
            br.set_endian(binary_reader::Endian::Little);

            // Check if it's an actual save file
            assert!(Self::is(&mut br));

            Self::read(&mut br)
        }

        // Check if it's a save file
        pub fn is(br: &mut BinaryReader) -> bool {
            let is = Self::is_pc(br) || Self::is_ps_save_wizard(br);
            is
        }

        // Check if it's a PC save file
        pub fn is_pc(br: &mut BinaryReader) -> bool {
            let magic = br.read_bytes(4).expect("");
            let is_pc = magic == [66, 78, 68, 52];
            br.jmp(0);
            is_pc
        }

        // Check if it's a PS Save Wizard save file
        pub fn is_ps_save_wizard(br: &mut BinaryReader) -> bool {
            br.jmp(0x1960080);
            let regulation = match br.read_bytes(0x1F1240) {
                Ok(bytes) => bytes,
                Err(_) => return false,
            };
            let is_ps_save_wizard = Regulation::check_save_compression(&regulation)
                .unwrap_or_else(|_| false);
            br.jmp(0);
            is_ps_save_wizard
        }
    }

}

#[cfg(test)]
mod tests {
    use crate::save::save::save::Save;
    use crate::write::write::Write;
    use std::path::PathBuf;

    fn diff_regions(a: &[u8], b: &[u8]) -> Vec<(usize, usize)> {
        let mut regions = Vec::new();
        let mut in_diff = false;
        let mut start = 0;
        for (i, (x, y)) in a.iter().zip(b.iter()).enumerate() {
            if x != y {
                if !in_diff { start = i; in_diff = true; }
            } else {
                if in_diff { regions.push((start, i - 1)); in_diff = false; }
            }
        }
        if in_diff { regions.push((start, a.len() - 1)); }
        regions
    }

    fn roundtrip(path: &str) {
        let path = PathBuf::from(path);
        let save = Save::from_path(&path).expect("failed to load save");
        let bytes = save.write().expect("failed to write save");
        let original = std::fs::read(&path).expect("failed to read original");
        let regions = diff_regions(&original, &bytes);
        println!("{}: diff_count={} regions={}", path.display(), original.iter().zip(bytes.iter()).filter(|(a, b)| a != b).count(), regions.len());
        for (s, e) in &regions {
            println!("  diff 0x{:x}-0x{:x} len=0x{:x}", s, e, e - s + 1);
        }
        assert!(regions.is_empty(), "round-trip should produce zero diffs");
    }

    #[test]
    fn test_ps_roundtrip_no_modify_v150() {
        roundtrip("~/ER-Save-Lib/test/PS_Save.txt");
    }

    #[test]
    fn test_ps_slot_versions() {
        let path = PathBuf::from("~/Documents/elden/memory.dat.2026-06-19_21-22-18");
        let save = Save::from_path(&path).expect("failed to load save");
        match &save.save_type {
            crate::SaveType::PlayStation(ps) => {
                for (i, slot) in ps.save_slots.iter().enumerate() {
                    println!("slot {}: ver={}", i, slot.ver);
                }
            }
            _ => {}
        }
    }

    #[test]
    fn test_ps_roundtrip_no_modify_v251() {
        roundtrip("~/Documents/elden/memory.dat.2026-06-19_21-22-18");
    }

    // End-to-end: add a key item through the inventory VM, persist, reload and
    // confirm (a) the item is present and (b) the only byte changes are the
    // intended inventory edits — no stray corruption elsewhere in the slot.
    #[test]
    fn test_ps_add_key_item_persists_and_confined() {
        use crate::vm::inventory::InventoryTypeRoute;
        use crate::vm::vm::vm::ViewModel;

        let path = PathBuf::from("~/Documents/elden/memory.dat.2026-06-19_21-22-18");
        let original = std::fs::read(&path).expect("failed to read original");

        let mut save = Save::from_path(&path).expect("failed to load save");
        let mut vm = ViewModel::from_save(&save);

        // First active character slot.
        let idx = save
            .save_type
            .active_slots()
            .iter()
            .position(|a| *a)
            .expect("expected at least one active slot");

        // Populate the key-item list from regulation and pick a real key item.
        vm.regulation.filter(&InventoryTypeRoute::KeyItems, "");
        let key_item = vm
            .regulation
            .filtered_goods
            .iter()
            .find(|i| i.is_key_item)
            .cloned()
            .expect("expected a key item in regulation params");
        let key_item_id = key_item.id;

        // Add it through the real VM path and commit to the save.
        vm.slots[idx].inventory_vm.add_to_inventory(&key_item);
        assert!(vm.slots[idx].inventory_vm.changed, "add should mark inventory changed");
        vm.update_save(&mut save.save_type);

        let bytes = save.write().expect("failed to write save");

        // Reload from the written bytes and confirm the key item survived.
        let tmp = std::env::temp_dir().join("er_save_editor_keyitem_test.dat");
        std::fs::write(&tmp, &bytes).expect("failed to write temp save");
        let save2 = Save::from_path(&tmp).expect("failed to reload save");
        let vm2 = ViewModel::from_save(&save2);
        let inv = &vm2.slots[idx].inventory_vm;
        let present = inv.storage[0].key_items.iter().any(|i| i.item_id == key_item_id)
            || inv.storage[1].key_items.iter().any(|i| i.item_id == key_item_id);
        assert!(present, "added key item {:#x} should be present after reload", key_item_id);
        let _ = std::fs::remove_file(&tmp);

        // The edit must change something, and only within a bounded set of regions
        // (inventory + counts), never a wholesale rewrite of the slot.
        let regions = diff_regions(&original, &bytes);
        let changed_bytes: usize = regions.iter().map(|(s, e)| e - s + 1).sum();
        println!(
            "add-key-item: regions={} changed_bytes={}",
            regions.len(),
            changed_bytes
        );
        assert!(!regions.is_empty(), "adding an item should change the save");
        assert!(
            changed_bytes < 0x1000,
            "unexpectedly large change ({} bytes) — possible corruption",
            changed_bytes
        );
    }

    // Toggling a v1.12+ summoning pool (block 670, previously wired to the placeholder
    // offset 0x0/bit0) must now map to a real, distinct event flag: it persists across a
    // save/reload and the change does not land on event_flags byte 0.
    #[test]
    fn test_ps_summoning_pool_670_toggle_persists() {
        use crate::db::summoning_pools::summoning_pools::SummoningPool;
        use crate::vm::vm::vm::ViewModel;

        let path = PathBuf::from("~/Documents/elden/memory.dat.2026-06-19_21-22-18");
        let original = std::fs::read(&path).expect("failed to read original");

        let mut save = Save::from_path(&path).expect("failed to load save");
        let mut vm = ViewModel::from_save(&save);
        let idx = save
            .save_type
            .active_slots()
            .iter()
            .position(|a| *a)
            .expect("expected at least one active slot");

        // Divine Tower of Caelid — the pool that was missing entirely from the table.
        let pool = SummoningPool::SummoningPool670490;
        let was_on = *vm.slots[idx]
            .events_vm
            .summoning_pools
            .get(&pool)
            .expect("pool should exist in events vm");
        let target = !was_on;
        vm.slots[idx].events_vm.summoning_pools.insert(pool, target);

        vm.update_save(&mut save.save_type);
        let bytes = save.write().expect("failed to write save");

        let tmp = std::env::temp_dir().join("er_save_editor_pool_test.dat");
        std::fs::write(&tmp, &bytes).expect("failed to write temp save");
        let save2 = Save::from_path(&tmp).expect("failed to reload save");
        let vm2 = ViewModel::from_save(&save2);
        let reloaded = *vm2.slots[idx]
            .events_vm
            .summoning_pools
            .get(&pool)
            .expect("pool should exist after reload");
        let _ = std::fs::remove_file(&tmp);
        assert_eq!(reloaded, target, "summoning pool toggle should persist");

        // The change must be small and must not corrupt event_flags byte 0 (the old
        // placeholder target). diff offsets are file offsets; just assert it's bounded.
        let regions = diff_regions(&original, &bytes);
        let changed_bytes: usize = regions.iter().map(|(s, e)| e - s + 1).sum();
        println!(
            "pool-toggle: regions={} changed_bytes={}",
            regions.len(),
            changed_bytes
        );
        assert!(!regions.is_empty(), "toggling a pool should change the save");
        assert!(
            changed_bytes < 0x40,
            "unexpectedly large change ({} bytes) for a single flag toggle",
            changed_bytes
        );
    }
}

