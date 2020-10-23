use std::convert::From;
use rayon::prelude::*;
use binary_stuff::*;
use die::*;

const REQUIRED_NUM_ARGS: usize = 1;
const DEFAULT_USE_BITS: &str = "zeros";
/// I wrote this for my own processor, 16 max threads, so divide
/// any file into 16 (if possible), TODO: make this a cli arg
const MAX_FILE_SEGMENTS: usize = 16;

#[derive(Copy, Clone)]
pub enum CountBit {
    CountBitZeros,
    CountBitOnes,
}
pub use CountBit::*;
impl From<&str> for CountBit {
    fn from(item: &str) -> Self {
        match item {
            "0" => CountBitZeros,
            "zeros" => CountBitZeros,
            "zero" => CountBitZeros,
            "1" => CountBitOnes,
            "ones" => CountBitOnes,
            "one" => CountBitOnes,
            _ => CountBitZeros,
        }
    }
}
impl ToString for CountBit {
    fn to_string(&self) -> String {
        let out_str = match self {
            CountBitOnes => "ones",
            CountBitZeros => "zeros",
        };
        out_str.into()
    }
}

pub fn read_binary_file(path: &str) -> Vec<u8> {
    let file_data = std::fs::read(path);
    if file_data.is_err() { die!("Failed to read {}", path) }
    file_data.unwrap()
}

// uses binary_stuff lib for the counting
pub fn get_max_data(data: &[u8], bit_count_type: CountBit) -> (usize, usize) {
    let zero_or_one = match bit_count_type {
        CountBitZeros => 0,
        CountBitOnes => 1,
    };
    // calculate a set of MAX_FILE_SEGMENTS, where each set
    // contains the indices of the data such that the set spans
    // the whole data. then use a rayon thread pool to iterate over
    // these indices in parallel and cound the max consecutive bits
    // of each file segment
    let mut thread_set = vec![];
    let data_bytes = data.len();
    let bytes_per_segment = data_bytes / MAX_FILE_SEGMENTS;
    for i in 0..MAX_FILE_SEGMENTS {
        let byte_index = i * bytes_per_segment;
        let bit_index = byte_index * 8;
        let next_i = i + 1;
        let next_segment_byte_index = next_i * bytes_per_segment;
        let this_segment_bytes = next_segment_byte_index - byte_index;
        let this_segment_bytes = if this_segment_bytes + byte_index > data_bytes {
            data_bytes - byte_index
        } else { this_segment_bytes };

        let this_segment_bits = this_segment_bytes * 8;
        thread_set.push((bit_index, this_segment_bits));
    }
    let joined_data: Vec<(usize, usize)> = thread_set.par_iter()
        .map(|thread_data| {
            // TODO: do I want
            // to add support for custom bin sizes? or
            // just use whole file size?
            binary_stuff::count_max_consecutive_bits(
                data,
                thread_data.1,
                thread_data.0,
                zero_or_one
            )
        })
        .collect();

    // after parallel work is done and collected into a vec
    // find the global max of all of the local maxes
    let mut max_consec = 0;
    let mut max_at = 0;
    for (consec, at) in joined_data {
        if consec > max_consec {
            max_consec = consec;
            max_at = at;
        }
    }

    (max_consec, max_at)
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    // shift args by 1. we dont care about the name of the program
    let args = &args[1..];
    let num_args = args.len();
    if num_args < REQUIRED_NUM_ARGS {
        die!("Must provide {} arguments. you provided {}", REQUIRED_NUM_ARGS, num_args);
    }

    // first arg is file name
    let file_name = &args[0];

    // second arg is if use 1s or 0s
    // by default use 0s
    let use_bits = match args.get(1) {
        Some(s) => s,
        None => DEFAULT_USE_BITS,
    };
    let use_bits: CountBit = use_bits.into();

    // TODO: optimize to read as stream, not all in one go
    let file_data = read_binary_file(file_name);
    let max_data = get_max_data(&file_data, use_bits);
    println!("Maximum {}: {}", use_bits.to_string(), max_data.0);
    println!("Occurred at bit index {}", max_data.1);
}
