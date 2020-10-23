# nbint

> **N**ikita's **Bin**ary **T**ools

A collection of tools that do some form of basic analysis on arbitrary files.

Currently there is only `nbint-consec` which will count the max number of consecutive bits of zeros (or ones) in a file.

## Installation

```sh
git clone https://github.com/nikita-skobov/nbint
cd nbint
cargo build --release
```

## Running

```sh
./targer/release/nbint-consec <path-to-file>
# or to count bits of 1 instead:
./targer/release/nbint-consec <path-to-file> 1
```


## Example

I used big buck bunny for this example: https://archive.org/details/BigBuckBunny_328 (I used the 512KB MPEG4 file)

```sh
[nbint@nbint]$ ./target/release/nbint-consec ~/Videos/BigBuckBunny_512kb.mp4
Maximum zeros: 267
Occurred at bit index 3768
```
