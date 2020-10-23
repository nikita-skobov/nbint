use std::convert::From;
use binary_stuff::*;
use die::*;

const REQUIRED_NUM_ARGS: usize = 1;
const DEFAULT_USE_BITS: &str = "zeros";

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
    // TODO: do I want
    // to add support for custom bin sizes? or
    // just use whole file size?
    binary_stuff::count_max_consecutive_bits(
        data,
        data.len() * 8,
        0,
        zero_or_one
    )
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
