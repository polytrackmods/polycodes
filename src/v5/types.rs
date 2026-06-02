use std::fmt::Display;

use num_enum::TryFromPrimitive;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TrackInfo {
    pub env: Environment,
    pub sun_dir: u8,

    pub min_x: i32,
    pub min_y: i32,
    pub min_z: i32,

    pub data_bytes: u8,
    pub parts: Vec<Part>,
}

#[derive(TryFromPrimitive, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Environment {
    Summer,
    Winter,
    Desert,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Part {
    pub id: u8,
    pub amount: u32,
    pub blocks: Vec<Block>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub x: u32,
    pub y: u32,
    pub z: u32,

    // why arent these combined into a single byte :( (literally takes up 5 bits in a span of 16 bits now)
    pub rotation: u8,
    pub dir: Direction,

    pub color: u8,
    pub cp_order: Option<u16>,
    pub start_order: Option<u32>,
}

#[derive(TryFromPrimitive, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum Direction {
    YPos,
    YNeg,
    XPos,
    XNeg,
    ZPos,
    ZNeg,
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Summer => write!(f, "Summer"),
            Self::Winter => write!(f, "Winter"),
            Self::Desert => write!(f, "Desert"),
        }
    }
}
