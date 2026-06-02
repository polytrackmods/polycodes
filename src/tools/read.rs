#[inline]
pub fn read_u8(buf: &[u8], offset: &mut usize) -> Option<u8> {
    let res = buf.get(*offset).copied();
    *offset += 1;
    res
}
#[inline]
pub fn read_u16(buf: &[u8], offset: &mut usize) -> Option<u16> {
    let res = Some(u16::from_le_bytes(
        buf.get(*offset..*offset + 2)?.try_into().ok()?,
    ));
    *offset += 2;
    res
}
#[inline]
pub fn read_i24(buf: &[u8], offset: &mut usize) -> Option<i32> {
    let res = Some(
        i32::from(*buf.get(*offset)?)
            | (i32::from(*buf.get(*offset + 1)?) << 8)
            | (i32::from(*buf.get(*offset + 2)?) << 16),
    );
    *offset += 3;
    res
}
#[inline]
pub fn read_u32(buf: &[u8], offset: &mut usize) -> Option<u32> {
    let res = Some(u32::from_le_bytes(
        buf.get(*offset..*offset + 4)?.try_into().ok()?,
    ));
    *offset += 4;
    res
}
