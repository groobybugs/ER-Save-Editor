pub mod stats_view_model {
    use crate::{db::classes::classes::ArcheType, save::common::save_slot::SaveSlot, vm::inventory::{InventoryGaitemType, FLASK_CERULEAN_BASE_ID, flask_upgrade_of_row}};

    #[derive(Clone)]
    pub struct StatsViewModel {
        pub arche_type: ArcheType,
        pub vigor: u32,
        pub mind: u32,
        pub endurance: u32,
        pub strength: u32,
        pub dexterity: u32,
        pub intelligence: u32,
        pub faith: u32,
        pub arcane: u32,
        pub level: u32,
        pub souls: u32,
        pub soulsmemory: u32,
        pub scadutree: u32,
        pub spirit_ash: u32,
        pub flask_hp: u32,
        pub flask_fp: u32,
        pub flask_upgrade: u32,
    }

    impl Default for StatsViewModel {
        fn default() -> Self {
            Self {
                arche_type: ArcheType::Unknown,
                vigor: Default::default(),
                mind: Default::default(),
                endurance: Default::default(),
                strength: Default::default(),
                dexterity: Default::default(),
                intelligence: Default::default(),
                faith: Default::default(),
                arcane: Default::default(),
                level: Default::default(),
                souls: Default::default(),
                soulsmemory: Default::default(),
                scadutree: Default::default(),
                spirit_ash: Default::default(),
                flask_hp: Default::default(),
                flask_fp: Default::default(),
                flask_upgrade: Default::default(),
            }
        }
    }

    impl StatsViewModel {
        pub fn from_save(slot: &SaveSlot) -> Self {
            // Unknown archetype bytes (e.g. classes added by later game
            // versions, such as the 1.17 Heavy/Idus Knights) must not crash
            // loading: fall back to Unknown instead of panicking.
            let arche_type =
                ArcheType::try_from(slot.player_game_data.arche_type).unwrap_or(ArcheType::Unknown);
            let vigor = slot.player_game_data.vigor;
            let mind = slot.player_game_data.mind;
            let endurance = slot.player_game_data.endurance;
            let strength = slot.player_game_data.strength;
            let dexterity = slot.player_game_data.dexterity;
            let intelligence = slot.player_game_data.intelligence;
            let faith = slot.player_game_data.faith;
            let arcane = slot.player_game_data.arcane;
            let level = slot.player_game_data.level;
            let souls = slot.player_game_data.souls;
            let soulsmemory = slot.player_game_data.soulsmemory;

            // DLC Stats
            let scadutree = slot.player_game_data.scadutree_lvl.into();
            let spirit_ash = slot.player_game_data.spirit_ash_lvl.into();

            // Flask charges (crimson HP / cerulean FP)
            let flask_hp = slot.player_game_data.flask_hp.into();
            let flask_fp = slot.player_game_data.flask_fp.into();

            // Flask upgrade level, read off the level-stepped flask goods
            // rows in held inventory (crimson preferred, else cerulean).
            let mut flask_upgrade = 0;
            for item in slot.equip_inventory_data.common_items.iter() {
                if (item.ga_item_handle & 0xf0000000) != InventoryGaitemType::ITEM as u32 {
                    continue;
                }
                let row = item.ga_item_handle ^ InventoryGaitemType::ITEM as u32;
                if let Some(level) = flask_upgrade_of_row(row) {
                    flask_upgrade = level;
                    if row < FLASK_CERULEAN_BASE_ID {
                        break;
                    }
                }
            }

            Self {
                arche_type,
                vigor,
                mind,
                endurance,
                strength,
                dexterity,
                intelligence,
                faith,
                arcane,
                level,
                souls,
                soulsmemory,
                scadutree,
                spirit_ash,
                flask_hp,
                flask_fp,
                flask_upgrade,
            }
        }
    }
}
