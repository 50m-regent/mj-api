use std::fmt;

pub enum Tile {
    M1 = 1 << 0, P1 = 1 << 9,  S1 = 1 << 18,
    M2 = 1 << 1, P2 = 1 << 10, S2 = 1 << 19,
    M3 = 1 << 2, P3 = 1 << 11, S3 = 1 << 20,
    M4 = 1 << 3, P4 = 1 << 12, S4 = 1 << 21,
    M5 = 1 << 4, P5 = 1 << 13, S5 = 1 << 22,
    M6 = 1 << 5, P6 = 1 << 14, S6 = 1 << 23,
    M7 = 1 << 6, P7 = 1 << 15, S7 = 1 << 24,
    M8 = 1 << 7, P8 = 1 << 16, S8 = 1 << 25,
    M9 = 1 << 8, P9 = 1 << 17, S9 = 1 << 26,

    East  = 1 << 27, White = 1 << 31,
    South = 1 << 28, Green = 1 << 32,
    West  = 1 << 29, Red   = 1 << 33,
    North = 1 << 30,

    Spring = 1 << 34, Plum          = 1 << 38,
    Summer = 1 << 35, Orchid        = 1 << 39,
    Autumn = 1 << 36, Chrysanthemum = 1 << 40,
    Winter = 1 << 37, Bamboo        = 1 << 41,

    IsRed   = 1 << 42,
    IsGold  = 1 << 43,
    IsBlack = 1 << 44,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::M1 => write!(f, "M1"), Tile::P1 => write!(f, "P1"), Tile::S1 => write!(f, "S1"),
            Tile::M2 => write!(f, "M2"), Tile::P2 => write!(f, "P2"), Tile::S2 => write!(f, "S2"),
            Tile::M3 => write!(f, "M3"), Tile::P3 => write!(f, "P3"), Tile::S3 => write!(f, "S3"),
            Tile::M4 => write!(f, "M4"), Tile::P4 => write!(f, "P4"), Tile::S4 => write!(f, "S4"),
            Tile::M5 => write!(f, "M5"), Tile::P5 => write!(f, "P5"), Tile::S5 => write!(f, "S5"),
            Tile::M6 => write!(f, "M6"), Tile::P6 => write!(f, "P6"), Tile::S6 => write!(f, "S6"),
            Tile::M7 => write!(f, "M7"), Tile::P7 => write!(f, "P7"), Tile::S7 => write!(f, "S7"),
            Tile::M8 => write!(f, "M8"), Tile::P8 => write!(f, "P8"), Tile::S8 => write!(f, "S8"),
            Tile::M9 => write!(f, "M9"), Tile::P9 => write!(f, "P9"), Tile::S9 => write!(f, "S9"),

            Tile::East  => write!(f, "Z1"), Tile::White => write!(f, "Z5"),
            Tile::South => write!(f, "Z2"), Tile::Green => write!(f, "Z6"),
            Tile::West  => write!(f, "Z3"), Tile::Red   => write!(f, "Z7"),
            Tile::North => write!(f, "Z4"),

            Tile::Spring => write!(f, "F1"), Tile::Plum          => write!(f, "F5"),
            Tile::Summer => write!(f, "F2"), Tile::Orchid        => write!(f, "F6"),
            Tile::Autumn => write!(f, "F3"), Tile::Chrysanthemum => write!(f, "F7"),
            Tile::Winter => write!(f, "F4"), Tile::Bamboo        => write!(f, "F8"),

            Tile::IsRed   => write!(f, "R"),
            Tile::IsGold  => write!(f, "G"),
            Tile::IsBlack => write!(f, "B"),
        }
    }
}