pub mod books {
    use std::{collections::HashMap, sync::Mutex};
    use once_cell::sync::Lazy;
    
    #[derive(PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
    pub enum Cookbook {
        // Missionary's Cookbook
        MissionarysCookbook1,
        MissionarysCookbook2,
        MissionarysCookbook3,
        MissionarysCookbook4,
        MissionarysCookbook5,
        MissionarysCookbook6,
        MissionarysCookbook7,

        // Nomadic warrior's Cookbook
        NomadicwarriorsCookbook1,
        NomadicwarriorsCookbook2,
        NomadicwarriorsCookbook3,
        NomadicwarriorsCookbook4,
        NomadicwarriorsCookbook5,
        NomadicwarriorsCookbook6,
        NomadicwarriorsCookbook7,
        NomadicwarriorsCookbook8,
        NomadicwarriorsCookbook9,
        NomadicwarriorsCookbook10,
        NomadicwarriorsCookbook11,
        NomadicwarriorsCookbook12,
        NomadicwarriorsCookbook13,
        NomadicwarriorsCookbook14,
        NomadicwarriorsCookbook15,
        NomadicwarriorsCookbook16,
        NomadicwarriorsCookbook17,
        NomadicwarriorsCookbook18,
        NomadicwarriorsCookbook19,
        NomadicwarriorsCookbook20,
        NomadicwarriorsCookbook21,
        NomadicwarriorsCookbook22,
        NomadicwarriorsCookbook23,
        NomadicwarriorsCookbook24,

        // Armorer's Cookbook
        ArmorersCookbook1,
        ArmorersCookbook2,
        ArmorersCookbook3,
        ArmorersCookbook4,
        ArmorersCookbook5,
        ArmorersCookbook6,
        ArmorersCookbook7,

        // Ancient Dragon Apostle's Cookbook
        AncientDragonApostlesCookbook1,
        AncientDragonApostlesCookbook2,
        AncientDragonApostlesCookbook3,
        AncientDragonApostlesCookbook4,

        // Fevor's Cookbook
        FevorsCookbook1,
        FevorsCookbook2,
        FevorsCookbook3,

        // Perfumer's Cookbook
        PerfumersCookbook1,
        PerfumersCookbook2,
        PerfumersCookbook3,
        PerfumersCookbook4,

        // Glintstone Craftman's Cookbook
        GlintstoneCraftmansCookbook1,
        GlintstoneCraftmansCookbook2,
        GlintstoneCraftmansCookbook3,
        GlintstoneCraftmansCookbook4,
        GlintstoneCraftmansCookbook5,
        GlintstoneCraftmansCookbook6,
        GlintstoneCraftmansCookbook7,
        GlintstoneCraftmansCookbook8,

        // Frenzied's Cookbook
        FrenziedsCookbook1,
        FrenziedsCookbook2,

        // Forager Brood Cookbook
        ForagerBroodCookbook1,
        ForagerBroodCookbook2,
        ForagerBroodCookbook3,
        ForagerBroodCookbook4,
        ForagerBroodCookbook5,
        ForagerBroodCookbook6,
        ForagerBroodCookbook7,

        // Igon's Cookbook
        IgonsCookbook1,
        IgonsCookbook2,

        // Finger-Weaver's Cookbook
        FingerWeaversCookbook1,
        FingerWeaversCookbook2,

        // Greater Potentate's Cookbook
        GreaterPotentatesCookbook1,
        GreaterPotentatesCookbook2,
        GreaterPotentatesCookbook3,
        GreaterPotentatesCookbook4,
        GreaterPotentatesCookbook5,
        GreaterPotentatesCookbook6,
        GreaterPotentatesCookbook7,
        GreaterPotentatesCookbook8,
        GreaterPotentatesCookbook9,
        GreaterPotentatesCookbook10,
        GreaterPotentatesCookbook11,
        GreaterPotentatesCookbook12,
        GreaterPotentatesCookbook13,
        GreaterPotentatesCookbook14,

        // Mad Craftsman's Cookbook
        MadCraftsmansCookbook1,
        MadCraftsmansCookbook2,
        MadCraftsmansCookbook3,

        // Ancient Dragon Knight's Cookbook
        AncientDragonKnightsCookbook1,
        AncientDragonKnightsCookbook2,

        // St Trina Disciple's Cookbook
        StTrinaDisciplesCookbook1,
        StTrinaDisciplesCookbook2,
        StTrinaDisciplesCookbook3,

        // Fire Knight's Cookbook
        FireKnightsCookbook1,
        FireKnightsCookbook2,

        // Loyal Knight's Cookbook
        LoyalKnightsCookbook,

        // Battlefield Priest's Cookbook
        BattlefieldPriestsCookbook1,
        BattlefieldPriestsCookbook2,
        BattlefieldPriestsCookbook3,
        BattlefieldPriestsCookbook4,

        // Grave Keeper's Cookbook
        GraveKeepersCookbook1,
        GraveKeepersCookbook2,

        // Antiquity Scholar's Cookbook
        AntiquityScholarsCookbook1,
        AntiquityScholarsCookbook2,

        // Tibia's Cookbook
        TibiasCookbook,
    }

    pub static COOKBOKS: Lazy<Mutex<HashMap<Cookbook,(u32,&str)>>> = Lazy::new(|| {
        Mutex::new(HashMap::from([
            // Missionary's Cookbook
            (Cookbook::MissionarysCookbook1,(67610,"Missionary's Cookbook[1]")),
            (Cookbook::MissionarysCookbook2,(67600,"Missionary's Cookbook[2]")),
            (Cookbook::MissionarysCookbook3,(67650,"Missionary's Cookbook[3]")),
            (Cookbook::MissionarysCookbook4,(67640,"Missionary's Cookbook[4]")),
            (Cookbook::MissionarysCookbook5,(67630,"Missionary's Cookbook[5]")),
            (Cookbook::MissionarysCookbook6,(67130,"Missionary's Cookbook[6]")),
            (Cookbook::MissionarysCookbook7,(68230,"Missionary's Cookbook[7]")),

            // Nomadic warrior's Cookbook
            (Cookbook::NomadicwarriorsCookbook1,(67000,"Nomadic warrior's Cookbook[1]")),
            (Cookbook::NomadicwarriorsCookbook2,(67110,"Nomadic warrior's Cookbook[2]")),
            (Cookbook::NomadicwarriorsCookbook3,(67010,"Nomadic warrior's Cookbook[3]")),
            (Cookbook::NomadicwarriorsCookbook4,(67800,"Nomadic warrior's Cookbook[4]")),
            (Cookbook::NomadicwarriorsCookbook5,(67830,"Nomadic warrior's Cookbook[5]")),
            (Cookbook::NomadicwarriorsCookbook6,(67020,"Nomadic warrior's Cookbook[6]")),
            (Cookbook::NomadicwarriorsCookbook7,(67050,"Nomadic warrior's Cookbook[7]")),
            (Cookbook::NomadicwarriorsCookbook8,(67880,"Nomadic warrior's Cookbook[8]")),
            (Cookbook::NomadicwarriorsCookbook9,(67430,"Nomadic warrior's Cookbook[9]")),
            (Cookbook::NomadicwarriorsCookbook10,(67030,"Nomadic warrior's Cookbook1[0]")),
            (Cookbook::NomadicwarriorsCookbook11,(67220,"Nomadic warrior's Cookbook1[1]")),
            (Cookbook::NomadicwarriorsCookbook12,(67060,"Nomadic warrior's Cookbook1[2]")),
            (Cookbook::NomadicwarriorsCookbook13,(67080,"Nomadic warrior's Cookbook1[3]")),
            (Cookbook::NomadicwarriorsCookbook14,(67870,"Nomadic warrior's Cookbook1[4]")),
            (Cookbook::NomadicwarriorsCookbook15,(67900,"Nomadic warrior's Cookbook1[5]")),
            (Cookbook::NomadicwarriorsCookbook16,(67290,"Nomadic warrior's Cookbook1[6]")),
            (Cookbook::NomadicwarriorsCookbook17,(67100,"Nomadic warrior's Cookbook1[7]")),
            (Cookbook::NomadicwarriorsCookbook18,(67270,"Nomadic warrior's Cookbook1[8]")),
            (Cookbook::NomadicwarriorsCookbook19,(67070,"Nomadic warrior's Cookbook1[9]")),
            (Cookbook::NomadicwarriorsCookbook20,(67230,"Nomadic warrior's Cookbook2[0]")),
            (Cookbook::NomadicwarriorsCookbook21,(67120,"Nomadic warrior's Cookbook2[1]")),
            (Cookbook::NomadicwarriorsCookbook22,(67890,"Nomadic warrior's Cookbook2[2]")),
            (Cookbook::NomadicwarriorsCookbook23,(67090,"Nomadic warrior's Cookbook2[3]")),
            (Cookbook::NomadicwarriorsCookbook24,(67910,"Nomadic warrior's Cookbook2[4]")),

            // Armorer's Cookbook
            (Cookbook::ArmorersCookbook1,(67200,"Armorer's Cookbook[1]")),
            (Cookbook::ArmorersCookbook2,(67210,"Armorer's Cookbook[2]")),
            (Cookbook::ArmorersCookbook3,(67280,"Armorer's Cookbook[3]")),
            (Cookbook::ArmorersCookbook4,(67260,"Armorer's Cookbook[4]")),
            (Cookbook::ArmorersCookbook5,(67310,"Armorer's Cookbook[5]")),
            (Cookbook::ArmorersCookbook6,(67300,"Armorer's Cookbook[6]")),
            (Cookbook::ArmorersCookbook7,(67250,"Armorer's Cookbook[7]")),

            // Ancient Dragon Apostle's Cookbook
            (Cookbook::AncientDragonApostlesCookbook1,(68000,"Ancient Dragon Apostle's Cookbook[1]")),
            (Cookbook::AncientDragonApostlesCookbook2,(68010,"Ancient Dragon Apostle's Cookbook[2]")),
            (Cookbook::AncientDragonApostlesCookbook3,(68030,"Ancient Dragon Apostle's Cookbook[3]")),
            (Cookbook::AncientDragonApostlesCookbook4,(68020,"Ancient Dragon Apostle's Cookbook[4]")),

            // Fevor's Cookbook
            (Cookbook::FevorsCookbook1,(68200,"Fevors Cookbook[1]")),
            (Cookbook::FevorsCookbook2,(68220,"Fevors Cookbook[2]")),
            (Cookbook::FevorsCookbook3,(68210,"Fevors Cookbook[3]")),

            // Perfumer's Cookbook
            (Cookbook::PerfumersCookbook1,(67840,"Perfumer's Cookbook[1]")),
            (Cookbook::PerfumersCookbook2,(67850,"Perfumer's Cookbook[2]")),
            (Cookbook::PerfumersCookbook3,(67860,"Perfumer's Cookbook[3]")),
            (Cookbook::PerfumersCookbook4,(67920,"Perfumer's Cookbook[4]")),

            // Glintstone Craftman's Cookbook
            (Cookbook::GlintstoneCraftmansCookbook1,(67410,"Glintstone Craftman's Cookbook[1]")),
            (Cookbook::GlintstoneCraftmansCookbook2,(67450,"Glintstone Craftman's Cookbook[2]")),
            (Cookbook::GlintstoneCraftmansCookbook3,(67480,"Glintstone Craftman's Cookbook[3]")),
            (Cookbook::GlintstoneCraftmansCookbook4,(67400,"Glintstone Craftman's Cookbook[4]")),
            (Cookbook::GlintstoneCraftmansCookbook5,(67420,"Glintstone Craftman's Cookbook[5]")),
            (Cookbook::GlintstoneCraftmansCookbook6,(67460,"Glintstone Craftman's Cookbook[6]")),
            (Cookbook::GlintstoneCraftmansCookbook7,(67470,"Glintstone Craftman's Cookbook[7]")),
            (Cookbook::GlintstoneCraftmansCookbook8,(67440,"Glintstone Craftman's Cookbook[8]")),

            // Frenzied's Cookbook
            (Cookbook::FrenziedsCookbook1,(68400,"Frenzied's Cookbook[1]")),
            (Cookbook::FrenziedsCookbook2,(68410,"Frenzied's Cookbook[2]")),

            // Greater Potentate's Cookbook
            (Cookbook::GreaterPotentatesCookbook1, (68590, "Greater Potentate's Cookbook[1]")),
            (Cookbook::GreaterPotentatesCookbook2, (68730, "Greater Potentate's Cookbook[2]")),
            (Cookbook::GreaterPotentatesCookbook3, (68690, "Greater Potentate's Cookbook[3]")),
            (Cookbook::GreaterPotentatesCookbook4, (68600, "Greater Potentate's Cookbook[4]")),
            (Cookbook::GreaterPotentatesCookbook5, (68610, "Greater Potentate's Cookbook[5]")),
            (Cookbook::GreaterPotentatesCookbook6, (68720, "Greater Potentate's Cookbook[6]")),
            (Cookbook::GreaterPotentatesCookbook7, (68630, "Greater Potentate's Cookbook[7]")),
            (Cookbook::GreaterPotentatesCookbook8, (68680, "Greater Potentate's Cookbook[8]")),
            (Cookbook::GreaterPotentatesCookbook9, (68640, "Greater Potentate's Cookbook[9]")),
            (Cookbook::GreaterPotentatesCookbook10, (68650, "Greater Potentate's Cookbook[10]")),
            (Cookbook::GreaterPotentatesCookbook11, (68660, "Greater Potentate's Cookbook[11]")),
            (Cookbook::GreaterPotentatesCookbook12, (68620, "Greater Potentate's Cookbook[12]")),
            (Cookbook::GreaterPotentatesCookbook13, (68700, "Greater Potentate's Cookbook[13]")),
            (Cookbook::GreaterPotentatesCookbook14, (68710, "Greater Potentate's Cookbook[14]")),

            // Mad Craftsman's Cookbook
            (Cookbook::MadCraftsmansCookbook1, (68750, "Mad Craftsman's Cookbook[1]")),
            (Cookbook::MadCraftsmansCookbook2, (68670, "Mad Craftsman's Cookbook[2]")),
            (Cookbook::MadCraftsmansCookbook3, (68880, "Mad Craftsman's Cookbook[3]")),

            // Ancient Dragon Knight's Cookbook
            (Cookbook::AncientDragonKnightsCookbook1, (68740, "Ancient Dragon Knight's Cookbook[1]")),
            (Cookbook::AncientDragonKnightsCookbook2, (68780, "Ancient Dragon Knight's Cookbook[2]")),

            // St Trina Disciple's Cookbook
            (Cookbook::StTrinaDisciplesCookbook1, (68760, "St Trina Disciple's Cookbook[1]")),
            (Cookbook::StTrinaDisciplesCookbook2, (68950, "St Trina Disciple's Cookbook[2]")),
            (Cookbook::StTrinaDisciplesCookbook3, (68840, "St Trina Disciple's Cookbook[3]")),

            // Forager Brood Cookbook
            (Cookbook::ForagerBroodCookbook1, (68520, "Forager Brood Cookbook[1]")),
            (Cookbook::ForagerBroodCookbook2, (68530, "Forager Brood Cookbook[2]")),
            (Cookbook::ForagerBroodCookbook3, (68540, "Forager Brood Cookbook[3]")),
            (Cookbook::ForagerBroodCookbook4, (68550, "Forager Brood Cookbook[4]")),
            (Cookbook::ForagerBroodCookbook5, (68560, "Forager Brood Cookbook[5]")),
            (Cookbook::ForagerBroodCookbook6, (68510, "Forager Brood Cookbook[6]")),
            (Cookbook::ForagerBroodCookbook7, (68830, "Forager Brood Cookbook[7]")),

            // Igon's Cookbook
            (Cookbook::IgonsCookbook1, (68810, "Igon's Cookbook[1]")),
            (Cookbook::IgonsCookbook2, (68570, "Igon's Cookbook[2]")),

            // Finger-Weaver's Cookbook
            (Cookbook::FingerWeaversCookbook1, (68920, "Finger-Weaver's Cookbook[1]")),
            (Cookbook::FingerWeaversCookbook2, (68580, "Finger-Weaver's Cookbook[2]")),

            // Fire Knight's Cookbook
            (Cookbook::FireKnightsCookbook1, (68770, "Fire Knight's Cookbook[1]")),
            (Cookbook::FireKnightsCookbook2, (68900, "Fire Knight's Cookbook[2]")),

            // Battlefield Priest's Cookbook
            (Cookbook::BattlefieldPriestsCookbook1, (68800, "Battlefield Priest's Cookbook[1]")),
            (Cookbook::BattlefieldPriestsCookbook2, (68820, "Battlefield Priest's Cookbook[2]")),
            (Cookbook::BattlefieldPriestsCookbook3, (68890, "Battlefield Priest's Cookbook[3]")),
            (Cookbook::BattlefieldPriestsCookbook4, (68930, "Battlefield Priest's Cookbook[4]")),

            // Grave Keeper's Cookbook
            (Cookbook::GraveKeepersCookbook1, (68940, "Grave Keeper's Cookbook[1]")),
            (Cookbook::GraveKeepersCookbook2, (68850, "Grave Keeper's Cookbook[2]")),

            // Antiquity Scholar's Cookbook
            (Cookbook::AntiquityScholarsCookbook1, (68910, "Antiquity Scholar's Cookbook[1]")),
            (Cookbook::AntiquityScholarsCookbook2, (68860, "Antiquity Scholar's Cookbook[2]")),

            // Tibia's Cookbook
            (Cookbook::TibiasCookbook, (68870, "Tibia's Cookbook")),

            // Loyal Knight's Cookbook
            (Cookbook::LoyalKnightsCookbook, (68790, "Loyal Knight's Cookbook")),
        ]))
    });
}