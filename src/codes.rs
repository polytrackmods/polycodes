use flate2::read::ZlibDecoder;
use sha2::{Digest, Sha256};
use std::io::Read;

use crate::base62;

/// Computes the track ID for a given track code. Returns [`None`] if something failed in the process.
pub fn export_to_id(track_code: &str) -> Option<String> {
    let track_data = decode_track_code(track_code)?;
    let id = hash_vec(track_data.2);
    Some(id)
}

/// Decodes the given track code and yields a tuple of the track name, track author, and the (raw binary) track data.
/// Returns [`None`] if something failed in the process.
pub fn decode_track_code(track_code: &str) -> Option<(String, String, Vec<u8>)> {
    // only use the actual data, skipping the "PolyTrack1"
    let track_code = track_code.get(10..)?;
    // ZLIB header 0x78DA is always encoded to `4p` and other stuff
    let td_start = track_code.find("4p")?;
    let track_data = track_code.get(td_start..)?;

    // (base64-decode and then decompress using zlib) x2
    let step1 = base62::decode(track_data)?;
    let step2 = decompress(&step1)?;
    let step2_str = String::from_utf8(step2).ok()?;
    let step3 = base62::decode(&step2_str)?;
    let step4 = decompress(&step3)?;

    let name_len = *step4.first()? as usize;
    let author_len = *step4.get(1 + name_len)? as usize;

    let name = String::from_utf8(step4.get(1..(1 + name_len))?.to_vec()).ok()?;
    let author = (if author_len > 0 {
        String::from_utf8(
            step4
                .get((name_len + 2)..(name_len + author_len + 2))?
                .to_vec(),
        )
        .ok()
    } else {
        None
    })?;
    let track_data = step4.get((name_len + author_len + 2)..)?.to_vec();

    Some((name, author, track_data))
}

fn decompress(data: &[u8]) -> Option<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut decompressed_data = Vec::new();
    decoder.read_to_end(&mut decompressed_data).ok()?;
    Some(decompressed_data)
}

fn hash_vec(track_data: Vec<u8>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(&track_data);
    let result = hasher.finalize();
    hex::encode(result)
}
