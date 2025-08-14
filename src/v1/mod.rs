use std::collections::HashMap;

use facet::Facet;

#[derive(Debug, Facet)]
struct JsonTrack {
    version: u32,
    name: String,
    track: String,
}

#[derive(Debug)]
pub struct Track {
    pub version: u32,
    pub name: String,
    pub track: TrackInfo,
}

#[derive(Debug, Facet)]
pub struct TrackInfo {
    pub version: u32,
    pub parts: HashMap<String, Vec<u32>>,
}

pub struct Block {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rot: u8,
}

pub fn decode_track_code(track_code: &str) -> Option<Track> {
    let json_track: JsonTrack = facet_json::from_str(track_code).ok()?;
    let track_info: TrackInfo = facet_json::from_str(&json_track.track).ok()?;
    Some(Track {
        version: json_track.version,
        name: json_track.name,
        track: track_info,
    })
}
