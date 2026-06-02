#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Track {
    pub version: u32,
    pub name: String,
    pub track: TrackInfo,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TrackInfo {
    pub version: u32,
    pub parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Part {
    pub id: u32,
    pub blocks: Vec<Block>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rot: u8,
}
