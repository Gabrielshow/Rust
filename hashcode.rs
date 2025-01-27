pub fn hashcode(bytes:  &[u8]) -> u32 {
    let mut a = 0_u32;
    for (i, b) in bytes.iter().enumerate() {
        a ^= *b as u32;
        a <<= i & 4;
    }
    a
}