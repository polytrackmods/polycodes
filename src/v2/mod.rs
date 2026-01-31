#![allow(clippy::cast_possible_truncation)]
#[cfg(test)]
mod tests;

use crate::tools::prelude::*;
use base64::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TrackInfo {
    pub parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Part {
    pub id: u8,
    pub amount: u32,
    pub blocks: Vec<Block>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub z: i32,

    pub rotation: u8,
}

fn decode(input: &str) -> Option<Vec<u8>> {
    let input = input.replace('-', "+").replace('_', "/");
    let base64_decoded = BASE64_STANDARD_NO_PAD.decode(input).ok()?;
    Some(base64_decoded)
}

fn encode(input: &[u8]) -> String {
    let base64_encoded = BASE64_STANDARD_NO_PAD.encode(input);
    base64_encoded.replace('+', "-").replace('/', "_")
}

#[must_use]
pub fn decode_track_code(track_code: &str) -> Option<Track> {
    let track_code = track_code.get(3..)?;
    let metadata = decode(track_code.get(..2)?)?;
    let name_len = *metadata.first()? as usize;
    let name = track_code.get(2..2 + name_len)?.to_string();
    let track_data = decode(track_code.get(2 + name_len..)?)?;
    Some(Track {
        name,
        author: None,
        track_data,
    })
}

#[must_use]
/// Encodes the given track struct into a track code.
/// Returns [`None`] if something failed in the process.
///
/// Output might differ slightly from Polytrack's output
/// because of Zlib shenanigans, but is still compatible.
pub fn encode_track_code(track: &Track) -> Option<String> {
    let track_data = encode(&track.track_data);

    let metadata = encode(&[(track.name.len()) as u8]);

    // prepend the "v1n"
    let track_code = String::from("v1n") + &metadata + &track.name + &track_data;
    Some(track_code)
}

#[must_use]
pub fn decode_track_data(data: &[u8]) -> Option<TrackInfo> {
    let mut offset = 0;
    let mut parts = Vec::new();
    while offset < data.len() {
        let id = read_u16(data, &mut offset)? as u8;
        let amount = read_u32(data, &mut offset)?;

        let mut blocks = Vec::new();
        for _ in 0..amount {
            let x = read_i24(data, &mut offset)? - i32::pow(2, 23);
            let y = read_i24(data, &mut offset)?;
            let z = read_i24(data, &mut offset)? - i32::pow(2, 23);

            let rotation = read_u8(data, &mut offset)? & 3;

            blocks.push(Block { x, y, z, rotation });
        }
        parts.push(Part { id, amount, blocks });
    }

    Some(TrackInfo { parts })
}

#[must_use]
/// Encodes the `TrackInfo` struct into raw binary data.
pub fn encode_track_data(track_info: &TrackInfo) -> Option<Vec<u8>> {
    let mut data = Vec::new();
    for part in &track_info.parts {
        write_u16(&mut data, part.id.into());
        write_u32(&mut data, part.amount);
        for block in &part.blocks {
            write_u24(&mut data, (block.x + i32::pow(2, 23)).cast_unsigned());
            write_u24(&mut data, block.y.cast_unsigned());
            write_u24(&mut data, (block.z + i32::pow(2, 23)).cast_unsigned());
            data.push(block.rotation);
        }
    }

    Some(data)
}
