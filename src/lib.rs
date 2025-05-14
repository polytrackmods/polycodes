use flate2::read::ZlibDecoder;
use sha2::{Digest, Sha256};
use std::io::Read;

pub fn export_to_id(track_code: &str) -> Option<String> {
    let track_data = decode_track_code(track_code)?;
    let id = hash_track_data(track_data);
    Some(id)
}

const I: [i32; 123] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51,
];

fn r(e: &mut Vec<u8>, t: usize, i: usize, n: i32, r: bool) {
    let s = t / 8;
    while s >= e.len() {
        e.push(0);
    }
    let a = t - 8 * s;
    e[s] |= ((n << a) & 0xFF) as u8;
    if a > 8 - i && !r {
        let t2 = s + 1;
        if t2 >= e.len() {
            e.push(0);
        }
        e[t2] |= (n >> (8 - a)) as u8;
    }
}

fn base62_decode(input: &str) -> Option<Vec<u8>> {
    let mut t = 0;
    let mut out: Vec<u8> = Vec::new();
    let s = input.len();

    for (a, ch) in input.chars().enumerate() {
        let o = ch as usize;
        if o >= I.len() {
            return None;
        }
        let l = I[o];
        if l == -1 {
            return None;
        }
        if (l & 30) == 30 {
            r(&mut out, t, 5, l, a == s - 1);
            t += 5;
        } else {
            r(&mut out, t, 6, l, a == s - 1);
            t += 6;
        }
    }
    Some(out)
}

fn decompress(data: &[u8]) -> Option<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut out = Vec::new();
    if decoder.read_to_end(&mut out).is_ok() {
        Some(out)
    } else {
        None
    }
}

fn decode_track_code(track_code: &str) -> Option<Vec<u8>> {
    let track_code = &track_code[10..];
    if let Some(td_start) = track_code.find("4p") {
        let track_data = &track_code[td_start..];
        let step1 = base62_decode(track_data)?;
        let step2 = decompress(&step1)?;
        let step2_str = String::from_utf8(step2).ok()?;
        let step3 = base62_decode(&step2_str)?;
        let step4 = decompress(&step3)?;

        let l = *step4.first()? as usize;
        let h = *step4.get(1 + l)? as usize;

        let data = step4.get((l + h + 2)..)?.to_vec();
        Some(data)
    } else {
        None
    }
}

fn hash_track_data(track_data: Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&track_data);
    let result = hasher.finalize();
    hex::encode(result)
}
