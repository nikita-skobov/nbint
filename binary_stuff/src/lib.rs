const BITS_IN_BYTE: usize = 8;


pub trait GetBitAt {
    fn get_bit_at(&self, pos: usize) -> u8;
    fn get_byte_at(&self, pos: usize) -> (u8, usize);
}

impl GetBitAt for [u8] {
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

/// given data, and an arbitrary bound of bit positions
/// ie: bin offset is the start, and bin size + bin offset is the end
/// find how many leading bits of zero or one are in that bound.
/// this function will handle overflow, eg: if you pass 1 byte, and try
/// to get leading zeros from bit offset 7 with bin size 4, then it will read
/// the 7th bit, and then overflow back to 0, 1, and 2.
pub fn count_leading_bits(
    data: &[u8],
    bin_size: usize,
    bit_offset: usize,
    zero_or_one: u8,
) -> usize {
    assert!(zero_or_one == 0 || zero_or_one == 1);

    // it is assumed that there can not be
    // overflow on the first bin size and offset,
    // ie: user who is calling this function is not
    // calling with bit offset > data_size_bits
    let data_size_bytes = data.len();
    let mut leading_bits = 0;
    let (mut current_byte, mut current_byte_index) = data.get_byte_at(bit_offset);
    let bit_index_of_byte = current_byte_index << 3; // multiply by 8
    let mut offset = bit_offset - bit_index_of_byte;

    for _ in 0..bin_size {
        let mask = 1 << (7 - offset);
        let val = current_byte & mask;
        let val = if val == 0 { 0 } else { 1 };
        if val == zero_or_one {
            leading_bits += 1;
        } else {
            break;
        }

        // calculate next byte/offset
        // by doing this calculation here,
        // we can prevent using the get_bit_at trait, which is slow
        // but here we keep the current byte cached for 8 ops, so much faster
        offset += 1;
        if offset == BITS_IN_BYTE {
            offset = 0;
            current_byte_index += 1;
            // handle overflow, ie: go back around to 0 if we pass the limit
            if current_byte_index == data_size_bytes {
                current_byte_index = 0;
            }
            current_byte = data[current_byte_index];
        }
    }
    leading_bits
}
pub fn count_leading_zeros(
    data: &[u8],
    bin_size: usize,
    bit_offset: usize,
) -> usize {
    count_leading_bits(data, bin_size, bit_offset, 0)
}
pub fn count_leading_ones(
    data: &[u8],
    bin_size: usize,
    bit_offset: usize,
) -> usize {
    count_leading_bits(data, bin_size, bit_offset, 1)
}


/// given some data, and the usual bin_size, bin_offset args,
/// iterate over the data and find the maximum amount of consecutive
/// bits of the bit_type. returns tuple of:
/// (max, where_max_occurs)
pub fn count_max_consecutive_bits(
    data: &[u8],
    bin_size: usize,
    bit_offset: usize,
    zero_or_one: u8,
) -> (usize, usize) {
    assert!(zero_or_one == 0 || zero_or_one == 1);
    // it is assumed that there can not be
    // overflow on the first bin size and offset,
    // ie: user who is calling this function is not
    // calling with bit offset > data_size_bits
    let data_size_bytes = data.len();
    let mut max_consec = 0;
    let mut max_at = bit_offset;
    let (mut current_byte, mut current_byte_index) = data.get_byte_at(bit_offset);
    let bit_index_of_byte = current_byte_index << 3; // multiply by 8
    let mut offset = bit_offset - bit_index_of_byte;
    let mut current_consec = 0;
    let mut current_at = max_at;

    for i in 0..bin_size {
        let mask = 1 << (7 - offset);
        let val = current_byte & mask;
        let val = if val == 0 { 0 } else { 1 };
        if val == zero_or_one {
            current_consec += 1;
        } else {
            if current_consec > max_consec {
                max_consec = current_consec;
                max_at = current_at;
            }

            current_consec = 0;
            // the plus 1 is because if our current
            // position is NOT the desired bit type
            // then the next index is the start of
            // the desired bit type
            current_at = bit_offset + i + 1;
        }

        // calculate next byte/offset
        // by doing this calculation here,
        // we can prevent using the get_bit_at trait, which is slow
        // but here we keep the current byte cached for 8 ops, so much faster
        offset += 1;
        if offset == BITS_IN_BYTE {
            offset = 0;
            current_byte_index += 1;
            // handle overflow, ie: go back around to 0 if we pass the limit
            if current_byte_index == data_size_bytes {
                current_byte_index = 0;
            }
            current_byte = data[current_byte_index];
        }
    }

    // if we ended the iteration and our current was greater than max
    if current_consec > max_consec {
        max_consec = current_consec;
        max_at = current_at;
    }

    (max_consec, max_at)
}
pub fn count_max_consecutive_zeros(
    data: &[u8],
    bin_size: usize,
    bit_offset: usize,
) -> (usize, usize) {
    count_max_consecutive_bits(data, bin_size, bit_offset, 0)
}
pub fn count_max_consecutive_ones(
    data: &[u8],
    bin_size: usize,
    bit_offset: usize,
) -> (usize, usize) {
    count_max_consecutive_bits(data, bin_size, bit_offset, 1)
}

#[cfg(test)]
mod tests {
    use super::count_zeros;
    use super::count_leading_bits;
    use super::count_max_consecutive_bits;

    #[test]
    fn count_max_consecutive_bits_works() {
        // max consec ones here   v
        let data = [0b01010101, 0b10101010, 0b01010101];
        // max consecutive zeros occurs here  ^
        let (max_zeros, at_zeros) = count_max_consecutive_bits(&data, data.len()*8, 0, 0);
        let (max_ones, at_ones) = count_max_consecutive_bits(&data, data.len()*8, 0, 1);
        assert_eq!(max_zeros, 2);
        assert_eq!(at_zeros, 15);
        assert_eq!(max_ones, 2);
        assert_eq!(at_ones, 7);

        let data = [0b11101111, 0b11111111];
        let (max_zeros, at_zeros) = count_max_consecutive_bits(&data, data.len()*8, 0, 0);
        let (max_ones, at_ones) = count_max_consecutive_bits(&data, data.len()*8, 0, 1);
        assert_eq!(max_zeros, 1);
        assert_eq!(at_zeros, 3);
        assert_eq!(max_ones, 12);
        assert_eq!(at_ones, 4);
    }

    #[test]
    fn count_leading_zeros_works() {
        let data = [0b00000000,0b00000000,0b00000000];
        assert_eq!(24, count_leading_bits(&data, data.len()*8, 0, 0));
        let data = [0b00000001,0b00000000,0b00000000];
        assert_eq!(7, count_leading_bits(&data, data.len()*8, 0, 0));
        // test that the overflow works. this will count 8 bits in the last byte
        // because offset is 16, and then it overflows to first bit of first byte
        // counts that as 9, and then breaks because next is a 1.
        let data = [0b01000001,0b00000000,0b00000000];
        assert_eq!(9, count_leading_bits(&data, data.len()*8, 16, 0));
    }

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
