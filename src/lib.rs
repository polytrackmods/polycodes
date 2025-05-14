use flate2::read::ZlibDecoder;
use sha2::{Digest, Sha256};
use std::io::Read;

pub fn export_to_id(track_code: &str) -> Option<String> {
    let track_data = decode_track_code(track_code)?;
    let id = hash_track_data(track_data);
    Some(id)
}

const CHAR_VALUES: [i32; 123] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51,
];

fn write_chars(
    bytes: &mut Vec<u8>,
    bit_index: usize,
    char_len: usize,
    char_value: i32,
    is_last: bool,
) {
    let byte_index = bit_index / 8;
    while byte_index >= bytes.len() {
        bytes.push(0);
    }

    let offset = bit_index - 8 * byte_index;
    bytes[byte_index] |= ((char_value << offset) & 0xFF) as u8;

    if offset > 8 - char_len && !is_last {
        let byte_index_next = byte_index + 1;
        if byte_index_next >= bytes.len() {
            bytes.push(0);
        }
        bytes[byte_index_next] |= (char_value >> (8 - offset)) as u8;
    }
}

fn base62_decode(input: &str) -> Option<Vec<u8>> {
    let mut out_pos = 0;
    let mut bytes_out: Vec<u8> = Vec::new();

    for (i, ch) in input.chars().enumerate() {
        let char_code = ch as usize;
        let char_value = *CHAR_VALUES.get(char_code)?;
        if char_value == -1 {
            return None;
        }
        // 5 if char_value is even and less than 32, 6 otherwise
        let char_len = if (char_value & 30) == 30 { 5 } else { 6 };
        write_chars(
            &mut bytes_out,
            out_pos,
            char_len,
            char_value,
            i == input.len() - 1,
        );
        out_pos += char_len;
    }

    Some(bytes_out)
}

fn decompress(data: &[u8]) -> Option<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).ok()?;
    Some(decompressed_data)
}

fn decode_track_code(track_code: &str) -> Option<Vec<u8>> {
    // only use the actual data, skipping the "PolyTrack1"
    let track_code = &track_code.get(10..)?;
    let td_start = track_code.find("4p")?;
    let track_data = &track_code.get(td_start..)?;

    // base64-decode and then decompress using zlib twice
    let step1 = base62_decode(track_data)?;
    let step2 = decompress(&step1)?;
    let step2_str = String::from_utf8(step2).ok()?;
    let step3 = base62_decode(&step2_str)?;
    let step4 = decompress(&step3)?;

    // remove data that is ignored in calculating track ID
    let l = *step4.first()? as usize;
    let h = *step4.get(1 + l)? as usize;
    let data = step4.get((l + h + 2)..)?.to_vec();

    Some(data)
}

fn hash_track_data(track_data: Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&track_data);
    let result = hasher.finalize();
    hex::encode(result)
}
