#[cfg(test)]
mod tests {
    use crc::{Crc, CRC_8_SMBUS};

    #[test]
    fn crc_8_smbus() {
        let x25: Crc<u8> = Crc::<u8>::new(&CRC_8_SMBUS);
        assert_eq!(x25.checksum(&[0xB6, 0x8D, 0xB7, 0xC0, 0xE8]), 0xC2);
    }
}
