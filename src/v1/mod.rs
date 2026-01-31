#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

use facet::Facet;

#[derive(Debug, Facet)]
struct JsonTrack {
    version: u32,
    name: String,
    track: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Track {
    pub version: u32,
    pub name: String,
    pub track: TrackInfo,
}

#[derive(Debug, Facet, PartialEq, Eq)]
pub struct JsonTrackInfo {
    pub version: u32,
    pub parts: BTreeMap<String, Vec<i32>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TrackInfo {
    pub version: u32,
    pub parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Part {
    pub id: u32,
    pub blocks: Vec<Block>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rot: u8,
}

#[must_use]
pub fn decode_track_code(track_code: &str) -> Option<Track> {
    let json_track: JsonTrack = facet_json::from_str(track_code).ok()?;
    let json_track_info: JsonTrackInfo = facet_json::from_str(&json_track.track).ok()?;
    let mut track_info = TrackInfo {
        version: json_track_info.version,
        parts: json_track_info
            .parts
            .into_iter()
            .map(|(id, blocks)| Part {
                id: id.parse().unwrap_or_default(),
                blocks: blocks
                    .chunks_exact(4)
                    .map(|block| Block {
                        x: block[0],
                        y: block[1],
                        z: block[2],
                        rot: block[3] as u8,
                    })
                    .collect(),
            })
            .collect(),
    };
    track_info.parts.sort_by_key(|part| part.id);
    Some(Track {
        version: json_track.version,
        name: json_track.name,
        track: track_info,
    })
}

#[must_use]
pub fn encode_track_code(track: Track) -> String {
    let parts = track
        .track
        .parts
        .into_iter()
        .map(|p| {
            format!(
                r#""{}":[{}]"#,
                p.id,
                p.blocks
                    .into_iter()
                    .flat_map(|b| vec![b.x, b.y, b.z, b.rot.into()])
                    .map(|n| n.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    let json_track_info = format!(r#"{{"version":{},"parts":{{{}}}}}"#, track.version, parts);
    let json_track = JsonTrack {
        version: track.version,
        name: track.name,
        track: json_track_info,
    };
    facet_json::to_string(&json_track).expect("should never fail")
}
