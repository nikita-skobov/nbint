const BITS_IN_BYTE: usize = 8;


pub trait GetBitAt {
    fn get_bit_at(&self, pos: usize) -> u8;
    fn get_byte_at(&self, pos: usize) -> (u8, usize);
}

impl GetBitAt for Vec<u8> {
    // slow but necessary for random access
    fn get_bit_at(&self, pos: usize) -> u8 {
        let (byte, byte_index) = self.get_byte_at(pos);
        let bit_index_of_byte = byte_index << 3; // multiply by 8
        let bit_offset = pos - bit_index_of_byte;
        let mask = 1 << (7 - bit_offset);
        let val = byte & mask;
        if val > 0 { 1 } else { 0 }
    }

    fn get_byte_at(&self, pos: usize) -> (u8, usize) {
        let byte_index = pos >> 3; // divide by 8
        (self[byte_index], byte_index)
    }
}

pub fn count_bits_internal(data: &[u8], zero_or_one: u8) -> u32 {
    assert!(zero_or_one == 1 || zero_or_one == 0);

    let mut num_bits = 0;
    let data_size = data.len();
    for i in 0..data_size {
        let byte = data[i];
        // here I could use the get_bit_at convenience trait
        // but its much slower because it needs to get the byte
        // into cache every call, which is a slow operation
        // here, the byte is cached for 8 iterations of j.
        // the benchmark on this is about 8 times faster
        for j in 0..BITS_IN_BYTE {
            let mask = 1 << (7 - j);
            let val = byte & mask;
            if val == zero_or_one {
                num_bits += 1;
            }
        }
    }
    num_bits
}
pub fn count_zeros(data: &[u8]) -> u32 {
    count_bits_internal(data, 0)
}
pub fn count_ones(data: &[u8]) -> u32 {
    count_bits_internal(data, 1)
}

#[cfg(test)]
mod tests {
    use super::count_zeros;

    #[test]
    fn count_zeros_works() {
        let data = vec![0b00000000, 0b00000000, 0b00000000];
        assert_eq!(24, count_zeros(&data));
    }

    #[test]
    fn bitwise_division_works() {
        let x = 16;
        assert_eq!(x >> 1, 8);
        assert_eq!(x >> 3, 2);
        let y: u8 = 128;
        assert_eq!(y >> 7, 1);

        assert_eq!(128 >> 3, 16);
        assert_eq!(129 >> 3, 16);
        assert_eq!(130 >> 3, 16);
        assert_eq!(131 >> 3, 16);
        assert_eq!(132 >> 3, 16);
        assert_eq!(133 >> 3, 16);
        assert_eq!(134 >> 3, 16);
        assert_eq!(135 >> 3, 16);
        assert_eq!(136 >> 3, 17);
    }
}
