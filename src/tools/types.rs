#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Track {
    pub name: String,
    pub author: Option<String>,
    pub last_modified: Option<u32>,
    pub track_data: Vec<u8>,
}
