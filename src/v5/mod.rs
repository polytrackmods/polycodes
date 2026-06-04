#![allow(clippy::cast_possible_wrap)]
#[cfg(test)]
mod tests;

use std::fmt::Display;

use num_enum::TryFromPrimitive;

use crate::tools::{self, prelude::*};
use crate::{Block, Part, Track};

pub const CP_IDS: [u8; 4] = [52, 65, 75, 77];
pub const START_IDS: [u8; 4] = [5, 91, 92, 93];

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct V5TrackInfo {
    author: String,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct V5Track {
    pub metadata: V5TrackMetadata,
    pub parts: Vec<V5Part>,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct V5TrackMetadata {
    pub env: V5Environment,
    pub sun_dir: u8,

    pub min_x: i32,
    pub min_y: i32,
    pub min_z: i32,

    pub data_bytes: u8,
}

#[derive(TryFromPrimitive, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum V5Environment {
    Summer,
    Winter,
    Desert,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct V5Part {
    pub id: u8,
    pub amount: u32,
    pub blocks: Vec<V5Block>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct V5Block {
    pub x: u32,
    pub y: u32,
    pub z: u32,

    pub rotation: u8,
    pub dir: V5Direction,

    pub color: u8,
    pub cp_order: Option<u16>,
    pub start_order: Option<u32>,
}

#[derive(TryFromPrimitive, Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum V5Direction {
    YPos,
    YNeg,
    XPos,
    XNeg,
    ZPos,
    ZNeg,
}

impl Track for V5Track {
    type Part = V5Part;

    type Metadata = V5TrackMetadata;
    fn meta(&self) -> Self::Metadata {
        self.metadata
    }
    fn parts(&self) -> Vec<Self::Part> {
        self.parts.clone()
    }

    fn decode_meta(data: &[u8], offset: &mut usize) -> Option<Self::Metadata> {
        let env = V5Environment::try_from(read_u8(data, offset)?).ok()?;
        let sun_dir = read_u8(data, offset)?;

        let min_x = read_u32(data, offset)?.cast_signed();
        let min_y = read_u32(data, offset)?.cast_signed();
        let min_z = read_u32(data, offset)?.cast_signed();

        let data_bytes = read_u8(data, offset)?;

        Some(V5TrackMetadata {
            env,
            sun_dir,

            min_x,
            min_y,
            min_z,

            data_bytes,
        })
    }
    fn encode_meta(&self, data: &mut Vec<u8>) {
        data.push(self.metadata.env as u8);
        data.push(self.metadata.sun_dir);
        write_u32(data, self.metadata.min_x.cast_unsigned());
        write_u32(data, self.metadata.min_y.cast_unsigned());
        write_u32(data, self.metadata.min_z.cast_unsigned());
        data.push(self.metadata.data_bytes);
    }
    fn from_data(metadata: Self::Metadata, parts: Vec<Self::Part>) -> Self {
        Self { metadata, parts }
    }

    type TrackInfo = V5TrackInfo;
    fn decode_track_code(track_code: &str) -> Option<(String, Self::TrackInfo, Vec<u8>)> {
        // only use the actual data, skipping the "PolyTrack2"
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
        let name = String::from_utf8(step4.get(1..=name_len)?.to_vec()).ok()?;

        let author_len = *step4.get(1 + name_len)? as usize;
        let author = String::from_utf8(
            step4
                .get((name_len + 2)..(name_len + author_len + 2))?
                .to_vec(),
        )
        .ok()?;
        let track_data = step4.get((name_len + author_len + 2)..)?.to_vec();

        Some((name, V5TrackInfo { author }, track_data))
    }
    fn encode_track_code(name: String, info: Self::TrackInfo, track_data: &[u8]) -> Option<String> {
        let mut data: Vec<u8> = Vec::new();

        let mut name = name.as_bytes().to_vec();
        data.push(name.len().try_into().ok()?);
        data.append(&mut name);

        let mut author = info.author.as_bytes().to_vec();
        data.push(author.len().try_into().ok()?);
        data.append(&mut author);

        data.append(&mut track_data.into());

        // (compress using zlib and then base62-encode) x2
        let step1 = tools::compress_first(&data)?;
        let step2_str = tools::encode(&step1)?;
        let step2 = step2_str.as_bytes();
        let step3 = tools::compress_final(step2)?;
        let step4 = tools::encode(&step3)?;

        // prepend the "PolyTrack1"
        let track_code = String::from("PolyTrack1") + &step4;
        Some(track_code)
    }
}
impl Part for V5Part {
    type Block = V5Block;
    fn id(&self) -> u8 {
        self.id
    }
    fn amount(&self) -> u32 {
        self.amount
    }
    fn blocks(&self) -> Vec<Self::Block> {
        self.blocks.clone()
    }
    fn from_data(id: u8, amount: u32, blocks: Vec<Self::Block>) -> Self {
        Self { id, amount, blocks }
    }

    fn decode_prelude(data: &[u8], offset: &mut usize) -> Option<(u8, u32)> {
        Some((read_u8(data, offset)? as u8, read_u32(data, offset)?))
    }
    fn encode_prelude(&self, data: &mut Vec<u8>) {
        data.push(self.id);
        write_u32(data, self.amount);
    }
}
impl Block for V5Block {
    type Track = V5Track;

    type Extra = (V5Direction, u8, Option<u16>, Option<u32>);
    fn extra_data(&self) -> Self::Extra {
        (self.dir, self.color, self.cp_order, self.start_order)
    }

    type Coord = u32;
    fn pos(&self) -> (Self::Coord, Self::Coord, Self::Coord) {
        (self.x, self.y, self.z)
    }
    fn rot(&self) -> u8 {
        self.rotation
    }

    fn decode(
        data: &[u8],
        offset: &mut usize,
        id: u8,
        meta: <Self::Track as Track>::Metadata,
    ) -> Option<Self> {
        let data_bytes = meta.data_bytes;
        let x_bytes = data_bytes & 3;
        let y_bytes = (data_bytes >> 2) & 3;
        let z_bytes = (data_bytes >> 4) & 3;

        let mut x = 0;
        for i in 0..x_bytes {
            x |= u32::from(*data.get(*offset + (i as usize))?) << (8 * i);
        }
        *offset += x_bytes as usize;

        let mut y = 0;
        for i in 0..y_bytes {
            y |= u32::from(*data.get(*offset + (i as usize))?) << (8 * i);
        }
        *offset += y_bytes as usize;

        let mut z = 0;
        for i in 0..z_bytes {
            z |= u32::from(*data.get(*offset + (i as usize))?) << (8 * i);
        }
        *offset += z_bytes as usize;

        let rotation = read_u8(data, offset)?;
        if rotation > 3 {
            return None;
        }
        let dir = V5Direction::try_from(read_u8(data, offset)?).ok()?;
        let color = read_u8(data, offset)?;
        // no custom color support for now
        if color > 3 && color < 32 && color > 40 {
            return None;
        }

        let cp_order = if CP_IDS.contains(&id) {
            Some(read_u16(data, offset)?)
        } else {
            None
        };
        let start_order = if START_IDS.contains(&id) {
            Some(read_u32(data, offset)?)
        } else {
            None
        };

        Some(Self {
            x,
            y,
            z,

            rotation,
            dir,

            color,
            cp_order,
            start_order,
        })
    }
    fn encode(&self, data: &mut Vec<u8>, meta: <Self::Track as Track>::Metadata) {
        let data_bytes = meta.data_bytes;
        let x_bytes = data_bytes & 3;
        let y_bytes = (data_bytes >> 2) & 3;
        let z_bytes = (data_bytes >> 4) & 3;

        match x_bytes {
            1 => write_u8(data, self.x),
            2 => write_u16(data, self.x),
            3 => write_u24(data, self.x),
            4 => write_u32(data, self.x),
            _ => {}
        }
        match y_bytes {
            1 => write_u8(data, self.y),
            2 => write_u16(data, self.y),
            3 => write_u24(data, self.y),
            4 => write_u32(data, self.y),
            _ => {}
        }
        match z_bytes {
            1 => write_u8(data, self.z),
            2 => write_u16(data, self.z),
            3 => write_u24(data, self.z),
            4 => write_u32(data, self.z),
            _ => {}
        }
        data.push(self.rotation);
        data.push(self.dir as u8);
        data.push(self.color);
        if let Some(cp_order) = self.cp_order {
            write_u16(data, cp_order.into());
        }
        if let Some(start_order) = self.start_order {
            write_u32(data, start_order);
        }
    }
}

impl Display for V5Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Summer => write!(f, "Summer"),
            Self::Winter => write!(f, "Winter"),
            Self::Desert => write!(f, "Desert"),
        }
    }
}
