# Bitonic Sorting Test for Rust Portable Simd

- Implements bitonic sorting networks for vectors of 8 or 16 values.
- Benchmarks them against `&mut [T].sort()`.

## Usage

- Use nightly channel of rustc.
- Run bench with `RUSTFLAGS="-C target-cpu=native" cargo bench`

## Results

AMD Ryzen 3 3100 CPU:

```bash
STD/f32/8               time: **98.062 µs**
STD/i32/8               time: **87.247 µs**
STD/i64/8               time: **90.890 µs**
STD/f32/16              time: **190.58 µs**
STD/i32/16              time: **145.36 µs**

BITONIC/f32/8           time: **77.812 µs**
BITONIC/i32/8           time: **48.640 µs**
BITONIC/i64/8           time: **119.05 µs**
BITONIC/f32/16          time: **215.24 µs**
BITONIC/i32/16          time: **159.63 µs**
```
