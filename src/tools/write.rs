#[inline]
pub fn write_u8(data: &mut Vec<u8>, value: u32) {
    data.push((value & 0xFF) as u8);
}
#[inline]
pub fn write_u16(data: &mut Vec<u8>, value: u32) {
    data.extend(((value & 0xFFFF) as u16).to_le_bytes());
}
#[inline]
pub fn write_u24(data: &mut Vec<u8>, value: u32) {
    data.push((value & 0xFF) as u8);
    data.push((value >> 8 & 0xFF) as u8);
    data.push((value >> 16 & 0xFF) as u8);
}
#[inline]
pub fn write_u32(data: &mut Vec<u8>, value: u32) {
    data.extend(value.to_le_bytes());
}
