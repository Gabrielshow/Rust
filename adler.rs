// this algorithm protects against random transmitting errors
// and cannot be used in authenticating messages;

const MOD_ADLER: u32 = 65521;

pub fn adler32(bytes: &[u8]) -> u32 {
    let mut a = 1_u32;
    let mut b = 0_u32;

    for byte in bytes {
        a = ( a + byte as u32) % MOD_ADLER;
        b = ( b + a ) % MOD_ADLER;
    }
    (b << 16) | a
}