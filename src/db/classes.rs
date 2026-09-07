pub mod classes {
    use std::{collections::HashMap, sync::Mutex};
    use once_cell::sync::Lazy;

    #[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord, Debug)]
    pub enum ArcheType {
        Unknown = -1,
        Vagabond = 0,
        Warrior = 1,
        Hero = 2,
        Bandit = 3,
        Astrologer = 4,
        Prophet = 5,
        Samurai = 7,
        Prisoner = 8,
        Confessor = 6,
        Wretch = 9,
        // 1.17 Tarnished Pack classes. IDs confirmed via the 1.17
        // regulation CharaInitParam: row 3010 = Idus Knight (level 7 +
        // Idus stats), row 3011 = Heavy Knight (level 10 + Heavy stats),
        // continuing the 3000+N class row order.
        IdusKnight = 10,
        HeavyKnight = 11,
    }

    impl TryFrom<u8> for ArcheType {
        type Error = ();
        fn try_from(v: u8) -> Result<Self, Self::Error> {
            match v {
                x if x == ArcheType::Vagabond as u8 => Ok(ArcheType::Vagabond),
                x if x == ArcheType::Warrior as u8 => Ok(ArcheType::Warrior),
                x if x == ArcheType::Hero as u8 => Ok(ArcheType::Hero),
                x if x == ArcheType::Bandit as u8 => Ok(ArcheType::Bandit),
                x if x == ArcheType::Astrologer as u8 => Ok(ArcheType::Astrologer),
                x if x == ArcheType::Prophet as u8 => Ok(ArcheType::Prophet),
                x if x == ArcheType::Samurai as u8 => Ok(ArcheType::Samurai),
                x if x == ArcheType::Prisoner as u8 => Ok(ArcheType::Prisoner),
                x if x == ArcheType::Confessor as u8 => Ok(ArcheType::Confessor),
                x if x == ArcheType::Wretch as u8 => Ok(ArcheType::Wretch),
                x if x == ArcheType::HeavyKnight as u8 => Ok(ArcheType::HeavyKnight),
                x if x == ArcheType::IdusKnight as u8 => Ok(ArcheType::IdusKnight),
                _ => Err(()),
            }
        }
    }

    impl From<ArcheType> for u8 {
        fn from(value: ArcheType) -> Self {
            match value {
                ArcheType::Unknown => 0xFF,
                ArcheType::Vagabond => ArcheType::Vagabond as u8,
                ArcheType::Warrior => ArcheType::Warrior as u8,
                ArcheType::Hero => ArcheType::Hero as u8,
                ArcheType::Bandit => ArcheType::Bandit as u8,
                ArcheType::Astrologer => ArcheType::Astrologer as u8,
                ArcheType::Prophet => ArcheType::Prophet as u8,
                ArcheType::Samurai => ArcheType::Samurai as u8,
                ArcheType::Prisoner => ArcheType::Prisoner as u8,
                ArcheType::Confessor => ArcheType::Confessor as u8,
                ArcheType::Wretch => ArcheType::Wretch as u8,
                ArcheType::HeavyKnight => ArcheType::HeavyKnight as u8,
                ArcheType::IdusKnight => ArcheType::IdusKnight as u8,
            }
        }
    }

    impl ToString for ArcheType {
        fn to_string(&self) -> String {
            match self {
                ArcheType::Unknown => "Unknown".to_string(),
                ArcheType::Vagabond => "Vagabond".to_string(),
                ArcheType::Warrior => "Warrior".to_string(),
                ArcheType::Hero => "Hero".to_string(),
                ArcheType::Bandit => "Bandit".to_string(),
                ArcheType::Astrologer => "Astrologer".to_string(),
                ArcheType::Prophet => "Prophet".to_string(),
                ArcheType::Samurai => "Samurai".to_string(),
                ArcheType::Prisoner => "Prisoner".to_string(),
                ArcheType::Confessor => "Confessor".to_string(),
                ArcheType::Wretch => "Wretch".to_string(),
                ArcheType::HeavyKnight => "Heavy Knight".to_string(),
                ArcheType::IdusKnight => "Idus Knight".to_string(),
            }
        }
    }

    #[derive(Clone, Copy)]
    pub struct Stats {
        pub level: u32,
        pub vigor: u32,
        pub mind: u32,
        pub endurance: u32,
        pub strength: u32,
        pub dexterity: u32,
        pub intelligence: u32,
        pub faith: u32,
        pub arcane: u32,
    }


    
    pub static STARTER_CLASSES: Lazy<Mutex<HashMap<ArcheType,Stats>>> = Lazy::new(|| {
        Mutex::new(HashMap::from([
            (ArcheType::Vagabond, Stats{
                level: 9,
                vigor: 15,
                mind: 10,
                endurance: 11,
                strength: 14,
                dexterity: 13,
                intelligence: 9,
                faith: 9,
                arcane: 7,
            }),
            
            (ArcheType::Warrior, Stats{
                level: 8,
                vigor: 11,
                mind: 12,
                endurance: 11,
                strength: 10,
                dexterity: 16,
                intelligence: 10,
                faith: 8,
                arcane: 9,
            }),
            
            (ArcheType::Hero, Stats{
                vigor: 14,
                mind: 9,
                endurance: 12,
                strength: 16,
                dexterity: 9,
                intelligence: 7,
                faith: 8,
                arcane: 11,
                level: 7,
            }),
            
            (ArcheType::Bandit, Stats{
                level: 5,
                vigor: 10,
                mind: 11,
                endurance: 10,
                strength: 9,
                dexterity: 13,
                intelligence: 9,
                faith: 8,
                arcane: 14,
            }),
            
            (ArcheType::Astrologer, Stats{
                level: 6,
                vigor: 9,
                mind: 15,
                endurance: 9,
                strength: 8,
                dexterity: 12,
                intelligence: 16,
                faith: 7,
                arcane: 9,
            }),
            
            (ArcheType::Prophet, Stats{
                level: 7,
                vigor: 10,
                mind: 14,
                endurance: 8,
                strength: 11,
                dexterity: 10,
                intelligence: 7,
                faith: 16,
                arcane: 10,
            }),
            
            (ArcheType::Samurai, Stats{
                level: 9,
                vigor: 12,
                mind: 11,
                endurance: 13,
                strength: 12,
                dexterity: 15,
                intelligence: 9,
                faith: 8,
                arcane: 8,
            }),
            
            (ArcheType::Prisoner, Stats{
                level: 9,
                vigor: 11,
                mind: 12,
                endurance: 11,
                strength: 11,
                dexterity: 14,
                intelligence: 14,
                faith: 6,
                arcane: 9,
            }),
            
            (ArcheType::Confessor, Stats{
                level: 10,
                vigor: 10,
                mind: 13,
                endurance: 10,
                strength: 12,
                dexterity: 12,
                intelligence: 9,
                faith: 14,
                arcane: 9,
            }),
            
            (ArcheType::Wretch, Stats{
                level: 1,
                vigor: 10,
                mind: 10,
                endurance: 10,
                strength: 10,
                dexterity: 10,
                intelligence: 10,
                faith: 10,
                arcane: 10,
            }),

            // 1.17 Tarnished Pack starters (stats per community wiki).
            (ArcheType::HeavyKnight, Stats{
                level: 10,
                vigor: 14,
                mind: 8,
                endurance: 17,
                strength: 15,
                dexterity: 11,
                intelligence: 7,
                faith: 8,
                arcane: 9,
            }),

            (ArcheType::IdusKnight, Stats{
                level: 7,
                vigor: 10,
                mind: 12,
                endurance: 11,
                strength: 13,
                dexterity: 15,
                intelligence: 8,
                faith: 11,
                arcane: 6,
            }),
        ]))
    });

    #[cfg(test)]
    mod tests {
        use super::{ArcheType, Stats, STARTER_CLASSES};

        #[test]
        fn new_117_classes_roundtrip_byte_values() {
            assert_eq!(ArcheType::try_from(10u8), Ok(ArcheType::IdusKnight));
            assert_eq!(ArcheType::try_from(11u8), Ok(ArcheType::HeavyKnight));
            assert_eq!(u8::from(ArcheType::IdusKnight), 10);
            assert_eq!(u8::from(ArcheType::HeavyKnight), 11);
            assert_eq!(ArcheType::HeavyKnight.to_string(), "Heavy Knight");
            assert_eq!(ArcheType::IdusKnight.to_string(), "Idus Knight");
        }

        #[test]
        fn base_game_class_ids_are_unchanged() {
            let expected = [
                (0, ArcheType::Vagabond),
                (1, ArcheType::Warrior),
                (2, ArcheType::Hero),
                (3, ArcheType::Bandit),
                (4, ArcheType::Astrologer),
                (5, ArcheType::Prophet),
                (6, ArcheType::Confessor),
                (7, ArcheType::Samurai),
                (8, ArcheType::Prisoner),
                (9, ArcheType::Wretch),
            ];
            for (byte, class) in expected {
                assert_eq!(ArcheType::try_from(byte), Ok(class));
                assert_eq!(u8::from(class), byte);
            }
        }

        fn starter_stats(class: ArcheType) -> Stats {
            *STARTER_CLASSES
                .lock()
                .unwrap()
                .get(&class)
                .expect("starter class definition missing")
        }

        #[test]
        fn heavy_knight_starter_stats_match_wiki() {
            let s = starter_stats(ArcheType::HeavyKnight);
            assert_eq!(
                (
                    s.level, s.vigor, s.mind, s.endurance, s.strength, s.dexterity,
                    s.intelligence, s.faith, s.arcane
                ),
                (10, 14, 8, 17, 15, 11, 7, 8, 9)
            );
        }

        #[test]
        fn idus_knight_starter_stats_match_wiki() {
            let s = starter_stats(ArcheType::IdusKnight);
            assert_eq!(
                (
                    s.level, s.vigor, s.mind, s.endurance, s.strength, s.dexterity,
                    s.intelligence, s.faith, s.arcane
                ),
                (7, 10, 12, 11, 13, 15, 8, 11, 6)
            );
        }

        #[test]
        fn unknown_archetype_fallback_never_panics() {
            // Bytes outside the known range must map to Err (callers fall
            // back to Unknown) rather than panic.
            assert!(ArcheType::try_from(12u8).is_err());
            assert!(ArcheType::try_from(0xFEu8).is_err());
        }
    }
}