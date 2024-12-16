#[derive(PartialEq, Debug)]
pub enum Tile {
    M1 = 1 << 0,  P1 = 1 << 12, S1 = 1 << 24,
    M2 = 1 << 1,  P2 = 1 << 13, S2 = 1 << 25,
    M3 = 1 << 2,  P3 = 1 << 14, S3 = 1 << 26,
    M4 = 1 << 3,  P4 = 1 << 15, S4 = 1 << 27,
    M5 = 1 << 4,  P5 = 1 << 16, S5 = 1 << 28,
    MR = 1 << 5,  PR = 1 << 17, SR = 1 << 29,
    MG = 1 << 6,  PG = 1 << 18, SG = 1 << 30,
    MB = 1 << 7,  PB = 1 << 19, SB = 1 << 31,
    M6 = 1 << 8,  P6 = 1 << 20, S6 = 1 << 32,
    M7 = 1 << 9,  P7 = 1 << 21, S7 = 1 << 33,
    M8 = 1 << 10, P8 = 1 << 22, S8 = 1 << 34,
    M9 = 1 << 11, P9 = 1 << 23, S9 = 1 << 35,

    East  = 1 << 36, White = 1 << 40,
    South = 1 << 37, Green = 1 << 41,
    West  = 1 << 38, Red   = 1 << 42,
    North = 1 << 39,

    Spring = 1 << 43, Plum          = 1 << 47,
    Summer = 1 << 44, Orchid        = 1 << 48,
    Autumn = 1 << 45, Chrysanthemum = 1 << 49,
    Winter = 1 << 46, Bamboo        = 1 << 50,
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Tile::M1 => write!(f, "M1"), Tile::P1 => write!(f, "P1"), Tile::S1 => write!(f, "S1"),
            Tile::M2 => write!(f, "M2"), Tile::P2 => write!(f, "P2"), Tile::S2 => write!(f, "S2"),
            Tile::M3 => write!(f, "M3"), Tile::P3 => write!(f, "P3"), Tile::S3 => write!(f, "S3"),
            Tile::M4 => write!(f, "M4"), Tile::P4 => write!(f, "P4"), Tile::S4 => write!(f, "S4"),
            Tile::M5 => write!(f, "M5"), Tile::P5 => write!(f, "P5"), Tile::S5 => write!(f, "S5"),
            Tile::MR => write!(f, "MR"), Tile::PR => write!(f, "PR"), Tile::SR => write!(f, "SR"),
            Tile::MG => write!(f, "MG"), Tile::PG => write!(f, "PG"), Tile::SG => write!(f, "SG"),
            Tile::MB => write!(f, "MB"), Tile::PB => write!(f, "PB"), Tile::SB => write!(f, "SB"),
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
        }
    }
}