pub fn decode_leb128<'a>(bytes: &'a Vec<u8>, pos: &mut usize, signed: bool) -> u64 {
    let mut result: u64 = 0;
    let mut shift: u8 = 0;
    loop {
        let b: u8 = bytes[*pos];
        result |= ((b & 0x7f) as u64) << shift;
        shift += 7;
        *pos += 1;
        if b & 0x80 == 0 {
            break;
        }
    }
    if signed && shift < 64 {
        result |= !(0 as u64) << shift;
    }
    result
}