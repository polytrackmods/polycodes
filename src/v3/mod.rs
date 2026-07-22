#[cfg(test)]
mod tests;

use crate::tools::{self, prelude::*};
use crate::{Block, Part, Track};

pub const CP_IDS: [u8; 4] = [52, 65, 75, 77];

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct V3Track {
    pub parts: Vec<V3Part>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct V3Part {
    pub id: u8,
    pub amount: u32,
    pub blocks: Vec<V3Block>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct V3Block {
    pub x: i32,
    pub y: i32,
    pub z: i32,

    pub rotation: u8,
    pub cp_order: Option<u16>,
}

impl Track for V3Track {
    type Part = V3Part;

    type Metadata = ();
    fn meta(&self) -> Self::Metadata {}
    fn parts(&self) -> Vec<Self::Part> {
        self.parts.clone()
    }

    fn decode_meta(_: &[u8], _: &mut usize) -> Option<Self::Metadata> {
        Some(())
    }
    fn encode_meta(&self, _: &mut Vec<u8>) {}
    fn from_data(_: Self::Metadata, parts: Vec<Self::Part>) -> Self {
        Self { parts }
    }

    type TrackInfo = ();
    fn decode_track_code(track_code: &str) -> Option<(String, Self::TrackInfo, Vec<u8>)> {
        let track_code = track_code.get(2..)?;
        let metadata = tools::decode(track_code.get(..2)?)?;
        let name_len_step1 = *metadata.first()? as usize;
        let name_len = (name_len_step1 * 4).div_ceil(3);
        let track_name_raw = tools::decode(track_code.get(2..2 + name_len)?)?;
        let name = String::from_utf8(track_name_raw).ok()?;
        let track_data = tools::decompress(&tools::decode(track_code.get(2 + name_len..)?)?)?;

        Some((name, (), track_data))
    }
    fn encode_track_code(name: String, _: Self::TrackInfo, track_data: &[u8]) -> Option<String> {
        let track_data = tools::encode(&tools::compress_final(track_data)?)?;

        let name_raw = name.as_bytes().to_vec();
        let name = tools::encode(&name_raw)?;
        let metadata = tools::encode(&[(name.len() * 3 / 4) as u8])?;

        // prepend the "v2"
        let track_code = String::from("v2") + &metadata + &name + &track_data;
        Some(track_code)
    }
}
impl Part for V3Part {
    type Block = V3Block;

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

    fn decode_header(data: &[u8], offset: &mut usize) -> Option<(u8, u32)> {
        Some((read_u16(data, offset)? as u8, read_u32(data, offset)?))
    }
    fn encode_header(&self, data: &mut Vec<u8>) {
        write_u16(data, self.id.into());
        write_u32(data, self.amount);
    }
}
impl Block for V3Block {
    type Track = V3Track;

    type Extra = Option<u16>;
    fn extra_data(&self) -> Self::Extra {
        self.cp_order
    }

    type Coord = i32;
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
        _: <Self::Track as Track>::Metadata,
    ) -> Option<Self> {
        let x = read_i24(data, offset)? - i32::pow(2, 23);
        let y = read_i24(data, offset)?;
        let z = read_i24(data, offset)? - i32::pow(2, 23);

        let rotation = read_u8(data, offset)? & 3;

        let cp_order = if CP_IDS.contains(&id) {
            Some(read_u16(data, offset)?)
        } else {
            None
        };

        Some(Self {
            x,
            y,
            z,
            rotation,
            cp_order,
        })
    }
    fn encode(&self, data: &mut Vec<u8>, _: <Self::Track as Track>::Metadata) {
        write_u24(data, (self.x + i32::pow(2, 23)).cast_unsigned());
        write_u24(data, self.y.cast_unsigned());
        write_u24(data, (self.z + i32::pow(2, 23)).cast_unsigned());
        data.push(self.rotation);
        if let Some(cp_order) = self.cp_order {
            write_u16(data, cp_order.into());
        }
    }
}
