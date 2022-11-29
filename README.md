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

## Annotated Example

```rust
pub fn bitonic_sort_i64x8(a: i64x4, b: i64x4) -> (i64x4, i64x4) {
    // a = [a0, a1, a2, a3]
    // b = [b0, b1, b2, b3]
    // =>
    // a = [a0, a3, b0, b3]
    // b = [a1, a2, b1, b2]
    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), First(3), Second(0), Second(3)]),
        swizzle!(a, b, [First(1), First(2), Second(1), Second(2)]),
    );

    // a = [a0, a3, b0, b3]
    // b = [a1, a2, b1, b2]
    // =>
    // a = [a0, a1, b2, b3]
    // b = [a2, a3, b0, b1]
    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), Second(0), Second(3), First(3)]),
        swizzle!(a, b, [Second(1), First(1), First(2), Second(2)]),
    );

    // a = [a0, a1, b2, b3]
    // b = [a2, a3, b0, b1]
    // =>
    // a = [a0, a2, b1, b3]
    // b = [a1, a3, b0, b2]
    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), Second(0), Second(3), First(3)]),
        swizzle!(a, b, [First(1), Second(1), Second(2), First(2)]),
    );


    // a = [a0, a2, b1, b3]
    // b = [a1, a3, b0, b2]
    // =>
    // a = [a0, a1, a2, a3]
    // b = [b0, b1, b2, b3]
    let (a, b) =  (
        swizzle!(a, b, [First(0), Second(0), First(1), Second(1)]),
        swizzle!(a, b, [Second(2), First(2), Second(3), First(3)]),
    );

    // At this point, the sequence is bitonic, meaning a0 <= a1 <= a2 <= a3 and b0 >= b1 >= b2 >= b3.
    // We can introduce a branch to compare a3 <= b3. If this is true, we can return early and just
    // reverse b.
    if a[3] <= b[3] {
        return (a, b.reverse());
    }

    let (a, b) = min_max!(a, b);

    // a = [a0, a1, a2, a3]
    // b = [b0, b1, b2, b3]
    // =>
    // a = [a0, a1, b0, b1]
    // b = [a2, a3, b2, b3]
    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), First(1), Second(0), Second(1)]),
        swizzle!(a, b, [First(2), First(3), Second(2), Second(3)]),
    );

    // a = [a0, a1, b0, b1]
    // b = [a2, a3, b2, b3]
    // =>
    // a = [a0, a2, b0, b2]
    // b = [a1, a3, b1, b3]
    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), Second(0), First(2), Second(2)]),
        swizzle!(a, b, [First(1), Second(1), First(3), Second(3)]),
    );

    // a = [a0, a2, b0, b2]
    // b = [a1, a3, b1, b3]
    // =>
    // a = [a0, a1, a2, a3]
    // b = [b0, b1, b2, b3]
    (
        swizzle!(a, b, [First(0), Second(0), First(1), Second(1)]),
        swizzle!(a, b, [First(2), Second(2), First(3), Second(3)]),
    )
}

pub fn bitonic_sort_f32x16(a: f32x8, b: f32x8) -> (f32x8, f32x8) {
    // a = [a0, a1, a2, a3, a4, a5, a6, a7]
    // b = [b0, b1, b2, b3, b4, b5, b6, b7]
    // =>
    // a = [a0, a3, a4, a7, b0, b3, b4, b7]
    // b = [a1, a2, a5, a6, b1, b2, b5, b6]

    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), First(3), First(4), First(7), Second(0), Second(3), Second(4), Second(7)]),
        swizzle!(a, b, [First(1), First(2), First(5), First(6), Second(1), Second(2), Second(5), Second(6)]),
    );

    // a = [a0, a3, a4, a7, b0, b3, b4, b7]
    // b = [a1, a2, a5, a6, b1, b2, b5, b6]
    // =>
    // a = [a0, a1, a7, a6, b0, b1, b7, b6]
    // b = [a2, a3, a5, a4, b2, b3, b5, b4]

    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), Second(0), First(3), Second(3), First(4), Second(4), First(7), Second(7)]),
        swizzle!(a, b, [Second(1), First(1), Second(2), First(2), Second(5), First(5), Second(6), First(6)]),
    );

    // a = [a0, a1, a7, a6, b0, b1, b7, b6]
    // b = [a2, a3, a5, a4, b2, b3, b5, b4]
    // =>
    // a = [a0, a2, a5, a7, b0, b2, b5, b7]
    // b = [a1, a3, a4, a6, b1, b3, b4, b6]

    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), Second(0), Second(2), First(2), First(4), Second(4), Second(6), First(6)]),
        swizzle!(a, b, [First(1), Second(1), Second(3), First(3), First(5), Second(5), Second(7), First(7)]),
    );

    // a = [a0, a2, a5, a7, b0, b2, b5, b7]
    // b = [a1, a3, a4, a6, b1, b3, b4, b6]
    // =>
    // a = [a0, a1, a2, a3, b7, b6, b5, b4]
    // b = [a4, a5, a6, a7, b3, b2, b1, b0]

    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), Second(0), First(1), Second(1), First(7), Second(7), First(6), Second(6)]),
        swizzle!(a, b, [Second(2), First(2), Second(3), First(3), Second(5), First(5), Second(4), First(4)]),
    );

    // a = [a0, a1, a2, a3, b7, b6, b5, b4]
    // b = [a4, a5, a6, a7, b3, b2, b1, b0]
    // =>
    // a = [a0, a1, a4, a5, b3, b2, b7, b6]
    // b = [a2, a3, a6, a7, b1, b0, b5, b4]

    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), First(1), Second(0), Second(1), Second(4), Second(5), First(4), First(5)]),
        swizzle!(a, b, [First(2), First(3), Second(2), Second(3), Second(6), Second(7), First(6), First(7)]),
    );

    // a = [a0, a1, a4, a5, b3, b2, b7, b6]
    // b = [a2, a3, a6, a7, b1, b0, b5, b4]
    // =>
    // a = [a0, a2, a4, a6, b1, b3, b5, b7]
    // b = [a1, a3, a5, a7, b0, b2, b4, b6]

    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), Second(0), First(2), Second(2), Second(4), First(4), Second(6), First(6)]),
        swizzle!(a, b, [First(1), Second(1), First(3), Second(3), Second(5), First(5), Second(7), First(7)]),
    );

    // a = [a0, a2, a4, a6, b1, b3, b5, b7]
    // b = [a1, a3, a5, a7, b0, b2, b4, b6]
    // =>
    // a = [a0, a1, a2, a3, a4, a5, a6, a7]
    // b = [b0, b1, b2, b3, b4, b5, b6, b7]
    let (a, b) = (
        swizzle!(a, b, [First(0), Second(0), First(1), Second(1), First(2), Second(2), First(3), Second(3)]),
        swizzle!(a, b, [Second(4), First(4), Second(5), First(5), Second(6), First(6), Second(7), First(7)]),
    );

    // At this point, the sequence is bitonic, meaning a0 <= .. <= a7 and b0 >= .. >= b7.
    // We can introduce a branch to compare a7 <= b7. If this is true, we can return early and just
    // reverse b.
    if a[7] <= b[7] {
        return (a, b.reverse());
    }

    let (a, b) = min_max!(a, b);

    // a = [a0, a1, a2, a3, a4, a5, a6, a7]
    // b = [b0, b1, b2, b3, b4, b5, b6, b7]
    // =>
    // a = [a0, a1, a2, a3, b0, b1, b2, b3]
    // b = [a4, a5, a6, a7, b4, b5, b6, b7]

    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), First(1), First(2), First(3), Second(0), Second(1), Second(2), Second(3)]),
        swizzle!(a, b, [First(4), First(5), First(6), First(7), Second(4), Second(5), Second(6), Second(7)]),
    );

    // a = [a0, a1, a2, a3, b0, b1, b2, b3]
    // b = [a4, a5, a6, a7, b4, b5, b6, b7]
    // =>
    // a = [a0, a1, a4, a5, b0, b1, b4, b5]
    // b = [a2, a3, a6, a7, b2, b3, b6, b7]

    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), First(1), Second(0), Second(1), First(4), First(5), Second(4), Second(5)]),
        swizzle!(a, b, [First(2), First(3), Second(2), Second(3), First(6), First(7), Second(6), Second(7)]),
    );

    // a = [a0, a1, a4, a5, b0, b1, b4, b5]
    // b = [a2, a3, a6, a7, b2, b3, b6, b7]
    // =>
    // a = [a0, a2, a4, a6, b0, b2, b4, b6]
    // b = [a1, a3, a5, a7, b1, b3, b5, b7]

    let (a, b) = min_max!(
        swizzle!(a, b, [First(0), Second(0), First(2), Second(2), First(4), Second(4), First(6), Second(6)]),
        swizzle!(a, b, [First(1), Second(1), First(3), Second(3), First(5), Second(5), First(7), Second(7)]),
    );

    // a = [a0, a2, a4, a6, b0, b2, b4, b6]
    // b = [a1, a3, a5, a7, b1, b3, b5, b7]
    // =>
    // a = [a0, a1, a2, a3, a4, a5, a6, a7]
    // b = [b0, b1, b2, b3, b4, b5, b6, b7]

    (
        swizzle!(a, b, [First(0), Second(0), First(1), Second(1), First(2), Second(2), First(3), Second(3)]),
        swizzle!(a, b, [First(4), Second(4), First(5), Second(5), First(6), Second(6), First(7), Second(7)]),
    )
}
```
