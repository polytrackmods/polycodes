#![allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
#[cfg(test)]
mod tests;

use crate::tools::{self, hash_vec, prelude::*};

pub const CP_IDS: [u8; 4] = [52, 65, 75, 77];

#[derive(Debug, PartialEq, Eq)]
pub struct TrackInfo {
    pub parts: Vec<Part>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Part {
    pub id: u8,
    pub amount: u32,
    pub blocks: Vec<Block>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub z: i32,

    pub rotation: u8,
    pub cp_order: Option<u16>,
}

#[must_use]
pub fn decode_track_code(track_code: &str) -> Option<Track> {
    let track_code = track_code.get(2..)?;
    let metadata = tools::decode(track_code.get(..2)?)?;
    let name_len_step1 = *metadata.first()? as usize;
    let name_len = (name_len_step1 * 4).div_ceil(3);
    let track_name_raw = tools::decode(track_code.get(2..2 + name_len)?)?;
    let name = String::from_utf8(track_name_raw).ok()?;
    let track_data = tools::decompress(&tools::decode(track_code.get(2 + name_len..)?)?)?;
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
    let track_data = tools::encode(&tools::compress(&track.track_data)?)?;

    let name_raw = track.name.as_bytes().to_vec();
    let name = tools::encode(&name_raw)?;
    let metadata = tools::encode(&[(name.len() * 3 / 4) as u8])?;

    // prepend the "v2"
    let track_code = String::from("v2") + &metadata + &name + &track_data;
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

            let cp_order = if CP_IDS.contains(&id) {
                Some(read_u16(data, &mut offset)?)
            } else {
                None
            };
            blocks.push(Block {
                x,
                y,
                z,
                rotation,
                cp_order,
            });
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
            if let Some(cp_order) = block.cp_order {
                write_u16(&mut data, cp_order.into());
            }
        }
    }

    Some(data)
}

#[must_use]
pub fn export_to_id(track_code: &str) -> Option<String> {
    let track_data = decode_track_code(track_code)?;
    let data = track_data.track_data;
    let id = hash_vec(data);
    Some(id)
}
