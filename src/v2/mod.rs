#[cfg(test)]
mod tests;

use base64::prelude::*;

use crate::tools::prelude::*;
use crate::{Block, Part, Track};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct V2Track {
    pub parts: Vec<V2Part>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct V2Part {
    pub id: u8,
    pub amount: u32,
    pub blocks: Vec<V2Block>,
}
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct V2Block {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub rotation: u8,
}

impl Track for V2Track {
    type Part = V2Part;

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
        let track_code = track_code.get(3..)?;
        let metadata = decode(track_code.get(..2)?)?;
        let name_len = *metadata.first()? as usize;
        let name = track_code.get(2..2 + name_len)?.to_string();
        let track_data = decode(track_code.get(2 + name_len..)?)?;

        Some((name, (), track_data))
    }
    fn encode_track_code(name: String, _: Self::TrackInfo, track_data: &[u8]) -> Option<String> {
        let track_data = encode(track_data);

        let metadata = encode(&[(name.len()) as u8]);

        // prepend the "v1n"
        let track_code = String::from("v1n") + &metadata + &name + &track_data;
        Some(track_code)
    }
}
impl Part for V2Part {
    type Block = V2Block;

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
impl Block for V2Block {
    type Track = V2Track;

    type Extra = ();
    fn extra_data(&self) -> Self::Extra {}

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
        _: u8,
        _: <Self::Track as Track>::Metadata,
    ) -> Option<Self> {
        let x = read_i24(data, offset)? - i32::pow(2, 23);
        let y = read_i24(data, offset)?;
        let z = read_i24(data, offset)? - i32::pow(2, 23);

        let rotation = read_u8(data, offset)? & 3;

        Some(Self { x, y, z, rotation })
    }
    fn encode(&self, data: &mut Vec<u8>, _: <Self::Track as Track>::Metadata) {
        write_u24(data, (self.x + i32::pow(2, 23)).cast_unsigned());
        write_u24(data, self.y.cast_unsigned());
        write_u24(data, (self.z + i32::pow(2, 23)).cast_unsigned());
        data.push(self.rotation);
    }
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
