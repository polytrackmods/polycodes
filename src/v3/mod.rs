use crate::tools::{self, Track, hash_vec};

pub const CP_IDS: [u8; 4] = [52, 65, 75, 77];

#[derive(Debug)]
pub struct TrackInfo {
    pub parts: Vec<Part>,
}

#[derive(Debug)]
pub struct Part {
    pub id: u8,
    pub amount: u32,
    pub blocks: Vec<Block>,
}

#[derive(Debug)]
pub struct Block {
    pub x: i32,
    pub y: i32,
    pub z: i32,

    pub rotation: u8,
    pub cp_order: Option<u16>,
}

pub fn decode_track_code(track_code: &str) -> Option<Track> {
    let track_code = track_code.get(2..)?;
    let metadata = tools::decode(track_code.get(..2)?)?;
    let name_len_step1 = *metadata.get(0)? as f64;
    let name_len = (name_len_step1 * 4.0 / 3.0).ceil() as usize;
    let track_name_raw = tools::decode(track_code.get(2..2 + name_len)?)?;
    let name = String::from_utf8(track_name_raw).ok()?;
    let track_data = tools::decompress(&tools::decode(track_code.get(2 + name_len..)?)?)?;
    Some(Track {
        name,
        author: None,
        track_data,
    })
}

pub fn decode_track_data(data: &[u8]) -> Option<TrackInfo> {
    #[inline]
    fn read_u8(buf: &[u8], offset: &mut usize) -> Option<u8> {
        let res = buf.get(*offset).copied();
        *offset += 1;
        res
    }
    #[inline]
    fn read_u16(buf: &[u8], offset: &mut usize) -> Option<u16> {
        let res = Some(*buf.get(*offset)? as u16 | ((*buf.get(*offset + 1)? as u16) << 8));
        *offset += 2;
        res
    }
    #[inline]
    fn read_u32(buf: &[u8], offset: &mut usize) -> Option<u32> {
        let res = Some(
            *buf.get(*offset)? as u32
                | ((*buf.get(*offset + 1)? as u32) << 8)
                | ((*buf.get(*offset + 2)? as u32) << 16)
                | ((*buf.get(*offset + 3)? as u32) << 24),
        );
        *offset += 4;
        res
    }
    #[inline]
    fn read_i24(buf: &[u8], offset: &mut usize) -> Option<i32> {
        let res = Some(
            *buf.get(*offset)? as i32
                | ((*buf.get(*offset + 1)? as i32) << 8)
                | ((*buf.get(*offset + 2)? as i32) << 16),
        );
        *offset += 3;
        res
    }

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

pub fn export_to_id(track_code: &str) -> Option<String> {
    let track_data = decode_track_code(track_code)?;
    let data = track_data.track_data;
    let id = hash_vec(data);
    Some(id)
}
