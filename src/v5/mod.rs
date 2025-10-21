#![allow(clippy::cast_possible_wrap)]
#[cfg(test)]
mod tests;

use std::fmt::Display;

use num_enum::TryFromPrimitive;

use crate::tools::{self, hash_vec, prelude::*};

pub const CP_IDS: [u8; 4] = [52, 65, 75, 77];
pub const START_IDS: [u8; 4] = [5, 91, 92, 93];

#[derive(Debug, PartialEq, Eq)]
pub struct TrackInfo {
    pub env: Environment,
    pub sun_dir: u8,

    pub min_x: i32,
    pub min_y: i32,
    pub min_z: i32,

    pub data_bytes: u8,
    pub parts: Vec<Part>,
}

#[derive(TryFromPrimitive, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Environment {
    Summer,
    Winter,
    Desert,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Part {
    pub id: u8,
    pub amount: u32,
    pub blocks: Vec<Block>,
}

#[derive(Debug, PartialEq, Eq)]
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

#[must_use]
/// Decodes the given track code and yields a struct containing the track name, track author, and the (raw binary) track data.
/// Returns [`None`] if something failed in the process.
pub fn decode_track_code(track_code: &str) -> Option<Track> {
    // only use the actual data, skipping the "PolyTrack1"
    let track_code = track_code.get(10..)?;
    // ZLIB header 0x78DA is always encoded to `4p` and other stuff
    let td_start = track_code.find("4p")?;
    let track_data = track_code.get(td_start..)?;

    // (base64-decode and then decompress using zlib) x2
    let step1 = tools::decode(track_data)?;
    let step2 = tools::decompress(&step1)?;
    let step2_str = String::from_utf8(step2).ok()?;
    let step3 = tools::decode(&step2_str)?;
    let step4 = tools::decompress(&step3)?;

    let name_len = *step4.first()? as usize;
    let author_len = *step4.get(1 + name_len)? as usize;

    let name = String::from_utf8(step4.get(1..=name_len)?.to_vec()).ok()?;
    let author = String::from_utf8(
        step4
            .get((name_len + 2)..(name_len + author_len + 2))?
            .to_vec(),
    )
    .ok();
    let track_data = step4.get((name_len + author_len + 2)..)?.to_vec();

    Some(Track {
        name,
        author,
        track_data,
    })
}

#[must_use]
/// Encodes the given track struct into a track code.
/// Returns [`None`] if something failed in the process.
///
/// Output might differ slightly from Polytrack's output
/// because of Zlib shenanigans, but is still compatible.
pub fn encode_track_code(track: Track) -> Option<String> {
    let mut data: Vec<u8> = Vec::new();

    let mut name = track.name.as_bytes().to_vec();
    data.push(name.len().try_into().ok()?);
    data.append(&mut name);

    if let Some(author) = track.author {
        let mut author = author.as_bytes().to_vec();
        data.push(author.len().try_into().ok()?);
        data.append(&mut author);
    } else {
        data.push(0);
    }

    data.append(&mut track.track_data.clone());

    // (compress using zlib and then base64-encode) x2
    let step1 = tools::compress(&data)?;
    let step2_str = tools::encode(&step1)?;
    let step2 = step2_str.as_bytes();
    let step3 = tools::compress(step2)?;
    let step4 = tools::encode(&step3)?;

    // prepend the "PolyTrack1"
    let track_code = String::from("PolyTrack1") + &step4;
    Some(track_code)
}

#[must_use]
/// Decodes the (raw binary) track data into a struct
/// representing everything that is in the data.
///
/// Fields of all involved structs correspond exactly to how
/// the data is stored in Polytrack itself.
/// Returns [`None`] if the data is not valid track data.
pub fn decode_track_data(data: &[u8]) -> Option<TrackInfo> {
    let mut offset = 0;

    let env = Environment::try_from(read_u8(data, &mut offset)?).ok()?;
    let sun_dir = read_u8(data, &mut offset)?;

    let min_x = read_u32(data, &mut offset)?.cast_signed();
    let min_y = read_u32(data, &mut offset)?.cast_signed();
    let min_z = read_u32(data, &mut offset)?.cast_signed();

    let data_bytes = read_u8(data, &mut offset)?;
    let x_bytes = data_bytes & 3;
    let y_bytes = (data_bytes >> 2) & 3;
    let z_bytes = (data_bytes >> 4) & 3;

    let mut parts = Vec::new();
    while offset < data.len() {
        let id = read_u8(data, &mut offset)?;
        let amount = read_u32(data, &mut offset)?;

        let mut blocks = Vec::new();
        for _ in 0..amount {
            let mut x = 0;
            for i in 0..x_bytes {
                x |= u32::from(*data.get(offset + (i as usize))?) << (8 * i);
            }
            offset += x_bytes as usize;

            let mut y = 0;
            for i in 0..y_bytes {
                y |= u32::from(*data.get(offset + (i as usize))?) << (8 * i);
            }
            offset += y_bytes as usize;

            let mut z = 0;
            for i in 0..z_bytes {
                z |= u32::from(*data.get(offset + (i as usize))?) << (8 * i);
            }
            offset += z_bytes as usize;

            let rotation = read_u8(data, &mut offset)?;
            if rotation > 3 {
                return None;
            }
            let dir = Direction::try_from(read_u8(data, &mut offset)?).ok()?;
            let color = read_u8(data, &mut offset)?;
            // no custom color support for now
            if color > 3 && color < 32 && color > 40 {
                return None;
            }

            let cp_order = if CP_IDS.contains(&id) {
                Some(read_u16(data, &mut offset)?)
            } else {
                None
            };
            let start_order = if START_IDS.contains(&id) {
                Some(read_u32(data, &mut offset)?)
            } else {
                None
            };

            blocks.push(Block {
                x,
                y,
                z,

                rotation,
                dir,

                color,
                cp_order,
                start_order,
            });
        }
        parts.push(Part { id, amount, blocks });
    }

    Some(TrackInfo {
        env,
        sun_dir,

        min_x,
        min_y,
        min_z,

        data_bytes,
        parts,
    })
}

#[must_use]
/// Encodes the `TrackInfo` struct into raw binary data.
pub fn encode_track_data(track_info: TrackInfo) -> Option<Vec<u8>> {
    let mut data = Vec::new();

    data.push(track_info.env as u8);
    data.push(track_info.sun_dir);
    write_u32(&mut data, track_info.min_x.cast_unsigned());
    write_u32(&mut data, track_info.min_y.cast_unsigned());
    write_u32(&mut data, track_info.min_z.cast_unsigned());
    data.push(track_info.data_bytes);
    let x_bytes = track_info.data_bytes & 3;
    let y_bytes = (track_info.data_bytes >> 2) & 3;
    let z_bytes = (track_info.data_bytes >> 4) & 3;
    for part in &track_info.parts {
        data.push(part.id);
        write_u32(&mut data, part.amount);
        for block in &part.blocks {
            match x_bytes {
                1 => write_u8(&mut data, block.x),
                2 => write_u16(&mut data, block.x),
                3 => write_u24(&mut data, block.x),
                4 => write_u32(&mut data, block.x),
                _ => {}
            }
            match y_bytes {
                1 => write_u8(&mut data, block.y),
                2 => write_u16(&mut data, block.y),
                3 => write_u24(&mut data, block.y),
                4 => write_u32(&mut data, block.y),
                _ => {}
            }
            match z_bytes {
                1 => write_u8(&mut data, block.z),
                2 => write_u16(&mut data, block.z),
                3 => write_u24(&mut data, block.z),
                4 => write_u32(&mut data, block.z),
                _ => {}
            }
            data.push(block.rotation);
            data.push(block.dir as u8);
            data.push(block.color);
            if let Some(cp_order) = block.cp_order {
                write_u16(&mut data, cp_order.into());
            }
            if let Some(start_order) = block.start_order {
                write_u32(&mut data, start_order);
            }
        }
    }

    Some(data)
}

#[must_use]
/// Computes the track ID for a given track code. Returns [`None`] if something failed in the process.
pub fn export_to_id(track_code: &str) -> Option<String> {
    let track_data = decode_track_code(track_code)?;
    let id = hash_vec(track_data.track_data);
    Some(id)
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
