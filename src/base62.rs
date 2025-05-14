const ENCODE_VALUES: [char; 62] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l',
    'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '0', '1', '2', '3', '4',
    '5', '6', '7', '8', '9',
];

const DECODE_VALUES: [i32; 123] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1, -1, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8,
    9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, -1, -1, -1, -1, -1, -1, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
    51,
];

/// Encode the givven byte buffer into base62 encoded text according to PolyTrack's base62 implementation.
/// Returns [`None`] if something failed in the process.
pub fn encode(input: &[u8]) -> Option<String> {
    let mut i = 0;
    let mut res = String::new();

    while i < 8 * input.len() {
        let s = encode_chars(input, i)?;
        let a;
        if (s & 30) == 30 {
            a = s & 31;
            i += 5;
        } else {
            a = s;
            i += 6;
        }
        res.push(*ENCODE_VALUES.get(a)?);
    }

    Some(res)
}

/// Decode the given string as base62 text according to PolyTrack's base62 implementation.
/// Returns [`None`] if any character isn't valid for base62 encoded text.
pub fn decode(input: &str) -> Option<Vec<u8>> {
    let mut out_pos = 0;
    let mut bytes_out: Vec<u8> = Vec::new();

    for (i, ch) in input.chars().enumerate() {
        let char_code = ch as usize;
        let char_value = *DECODE_VALUES.get(char_code)?;
        if char_value == -1 {
            return None;
        }
        // 5 if char_value is 30 or 31, 6 otherwise (don't ask me why)
        let value_len = if (char_value & 30) == 30 { 5 } else { 6 };
        decode_chars(
            &mut bytes_out,
            out_pos,
            value_len,
            char_value,
            i == input.len() - 1,
        );
        out_pos += value_len;
    }

    Some(bytes_out)
}

fn encode_chars(bytes: &[u8], t: usize) -> Option<usize> {
    if t >= 8 * bytes.len() {
        return None;
    }

    let i = t / 8;
    let n = *bytes.get(i)? as usize;
    let r = t - 8 * i;
    if r <= 2 || i >= bytes.len() - 1 {
        Some((n & (63 << r)) >> r)
    } else {
        Some((n & (63 << r)) >> r | (((*bytes.get(i + 1)? as usize) & (63 >> (8 - r))) << (8 - r)))
    }
}

fn decode_chars(
    bytes: &mut Vec<u8>,
    bit_index: usize,
    value_len: usize,
    char_value: i32,
    is_last: bool,
) {
    let byte_index = bit_index / 8;
    while byte_index >= bytes.len() {
        bytes.push(0);
    }

    // offset in current byte
    let offset = bit_index - 8 * byte_index;

    // writes value into byte (only part that fits)
    bytes[byte_index] |= ((char_value << offset) & 0xFF) as u8;

    // in case of value going into next byte add that part
    if offset > 8 - value_len && !is_last {
        let byte_index_next = byte_index + 1;
        if byte_index_next >= bytes.len() {
            bytes.push(0);
        }

        // write rest of value into next byte
        bytes[byte_index_next] |= (char_value >> (8 - offset)) as u8;
    }
}
