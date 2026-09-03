pub struct GestureInfo {
    pub index: usize,
    pub id: i32,
    pub name: &'static str,
}

pub const GESTURES: &[GestureInfo] = &[
    GestureInfo { index: 0, id: 1, name: "Bow" },
    GestureInfo { index: 1, id: 3, name: "Polite Bow" },
    GestureInfo { index: 2, id: 5, name: "My Thanks" },
    GestureInfo { index: 3, id: 7, name: "Curtsy" },
    GestureInfo { index: 4, id: 9, name: "Reverential Bow" },
    GestureInfo { index: 5, id: 11, name: "My Lord" },
    GestureInfo { index: 6, id: 13, name: "Warm Welcome" },
    GestureInfo { index: 7, id: 15, name: "Wave" },
    GestureInfo { index: 8, id: 17, name: "Casual Greeting" },
    GestureInfo { index: 9, id: 19, name: "Strength!" },
    GestureInfo { index: 10, id: 21, name: "As You Wish" },
    GestureInfo { index: 11, id: 41, name: "Point Forwards" },
    GestureInfo { index: 12, id: 43, name: "Point Upwards" },
    GestureInfo { index: 13, id: 45, name: "Point Downwards" },
    GestureInfo { index: 14, id: 47, name: "Beckon" },
    GestureInfo { index: 15, id: 49, name: "Wait!" },
    GestureInfo { index: 16, id: 51, name: "Calm Down!" },
    GestureInfo { index: 17, id: 61, name: "Nod In Thought" },
    GestureInfo { index: 18, id: 81, name: "Extreme Repentance" },
    GestureInfo { index: 19, id: 83, name: "Grovel For Mercy" },
    GestureInfo { index: 20, id: 101, name: "Rallying Cry" },
    GestureInfo { index: 21, id: 103, name: "Heartening Cry" },
    GestureInfo { index: 22, id: 105, name: "By My Sword" },
    GestureInfo { index: 23, id: 107, name: "Hoslow's Oath" },
    GestureInfo { index: 24, id: 109, name: "Fire Spur Me" },
    GestureInfo { index: 25, id: 111, name: "The Carian Oath (Cut)" }, // -1 in Lua, assuming 0 or skip
    GestureInfo { index: 26, id: 121, name: "Bravo!" },
    GestureInfo { index: 27, id: 141, name: "Jump for Joy" },
    GestureInfo { index: 28, id: 143, name: "Triumphant Delight" },
    GestureInfo { index: 29, id: 145, name: "Fancy Spin" },
    GestureInfo { index: 30, id: 147, name: "Finger Snap" },
    GestureInfo { index: 31, id: 161, name: "Dejection" },
    GestureInfo { index: 32, id: 181, name: "Patches' Crouch" },
    GestureInfo { index: 33, id: 183, name: "Crossed Legs" },
    GestureInfo { index: 34, id: 185, name: "Rest" },
    GestureInfo { index: 35, id: 187, name: "Sitting Sideways" },
    GestureInfo { index: 36, id: 189, name: "Dozing Cross-Legged" },
    GestureInfo { index: 37, id: 191, name: "Spread Out" },
    GestureInfo { index: 38, id: 193, name: "Fetal Position (Cut)" },
    GestureInfo { index: 39, id: 195, name: "Balled Up" },
    GestureInfo { index: 40, id: 197, name: "What Do You Want?" },
    GestureInfo { index: 41, id: 201, name: "Prayer" },
    GestureInfo { index: 42, id: 203, name: "Desperate Prayer" },
    GestureInfo { index: 43, id: 205, name: "Rapture" },
    GestureInfo { index: 44, id: 207, name: "Erudition" },
    GestureInfo { index: 45, id: 209, name: "Outer Order" },
    GestureInfo { index: 46, id: 211, name: "Inner Order" },
    GestureInfo { index: 47, id: 213, name: "Golden Order Totality" },
    GestureInfo { index: 48, id: 217, name: "The Ring (Pre-order)" }, // Index 49 in Lua (1-based) -> 48 Rust. ID 217
    GestureInfo { index: 49, id: 219, name: "The Ring" }, // Index 50 in Lua -> 49 Rust. ID 219
    GestureInfo { index: 50, id: 221, name: "?GoodsName? (Cut)" },
    
    // DLC Gestures appended?
    // Lua: gesturesDLC1
    GestureInfo { index: 51, id: 223, name: "May the Best Win" },
    GestureInfo { index: 52, id: 225, name: "The Two Fingers" },
    GestureInfo { index: 53, id: 227, name: "Ring of Miquella (Pre-order)" }, // ID 227 if owned?
    GestureInfo { index: 54, id: 229, name: "Let Us Go Together" },
    GestureInfo { index: 55, id: 231, name: "O Mother" },
    GestureInfo { index: 56, id: 233, name: "Ring of Miquella" }, // ID 233
];

