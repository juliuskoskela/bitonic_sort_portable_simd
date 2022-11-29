use super::*;

pub fn bitonic_sort_f32x8(a: f32x4, b: f32x4) -> (f32x4, f32x4) {
	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(3), B(0), B(3)]),
		swizzle!(a, b, [A(1), A(2), B(1), B(2)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), B(3), A(3)]),
		swizzle!(a, b, [B(1), A(1), A(2), B(2)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), B(3), A(3)]),
		swizzle!(a, b, [A(1), B(1), B(2), A(2)]),
	);

	let (a, b) =  (
		swizzle!(a, b, [A(0), B(0), A(1), B(1)]),
		swizzle!(a, b, [B(2), A(2), B(3), A(3)]),
	);

	if a[3] <= b[3] {
		return (a, b.reverse());
	}

	let (a, b) = min_max!(a, b);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), B(0), B(1)]),
		swizzle!(a, b, [A(2), A(3), B(2), B(3)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(2), B(2)]),
		swizzle!(a, b, [A(1), B(1), A(3), B(3)]),
	);

	(
		swizzle!(a, b, [A(0), B(0), A(1), B(1)]),
		swizzle!(a, b, [A(2), B(2), A(3), B(3)]),
	)
}

pub fn bitonic_sort_i32x8(a: i32x4, b: i32x4) -> (i32x4, i32x4) {
	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(3), B(0), B(3)]),
		swizzle!(a, b, [A(1), A(2), B(1), B(2)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), B(3), A(3)]),
		swizzle!(a, b, [B(1), A(1), A(2), B(2)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), B(3), A(3)]),
		swizzle!(a, b, [A(1), B(1), B(2), A(2)]),
	);

	let (a, b) =  (
		swizzle!(a, b, [A(0), B(0), A(1), B(1)]),
		swizzle!(a, b, [B(2), A(2), B(3), A(3)]),
	);

	if a[3] <= b[3] {
		return (a, b.reverse());
	}

	let (a, b) = min_max!(a, b);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), B(0), B(1)]),
		swizzle!(a, b, [A(2), A(3), B(2), B(3)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(2), B(2)]),
		swizzle!(a, b, [A(1), B(1), A(3), B(3)]),
	);

	(
		swizzle!(a, b, [A(0), B(0), A(1), B(1)]),
		swizzle!(a, b, [A(2), B(2), A(3), B(3)]),
	)
}

pub fn bitonic_sort_i64x8(a: i64x4, b: i64x4) -> (i64x4, i64x4) {
	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(3), B(0), B(3)]),
		swizzle!(a, b, [A(1), A(2), B(1), B(2)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), B(3), A(3)]),
		swizzle!(a, b, [B(1), A(1), A(2), B(2)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), B(3), A(3)]),
		swizzle!(a, b, [A(1), B(1), B(2), A(2)]),
	);

	let (a, b) =  (
		swizzle!(a, b, [A(0), B(0), A(1), B(1)]),
		swizzle!(a, b, [B(2), A(2), B(3), A(3)]),
	);

	if a[3] <= b[3] {
		return (a, b.reverse());
	}

	let (a, b) = min_max!(a, b);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), B(0), B(1)]),
		swizzle!(a, b, [A(2), A(3), B(2), B(3)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(2), B(2)]),
		swizzle!(a, b, [A(1), B(1), A(3), B(3)]),
	);

	(
		swizzle!(a, b, [A(0), B(0), A(1), B(1)]),
		swizzle!(a, b, [A(2), B(2), A(3), B(3)]),
	)
}

pub fn bitonic_sort_f32x16(a: f32x8, b: f32x8) -> (f32x8, f32x8) {
	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(3), A(4), A(7), B(0), B(3), B(4), B(7)]),
		swizzle!(a, b, [A(1), A(2), A(5), A(6), B(1), B(2), B(5), B(6)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(3), B(3), A(4), B(4), A(7), B(7)]),
		swizzle!(a, b, [B(1), A(1), B(2), A(2), B(5), A(5), B(6), A(6)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), B(2), A(2), A(4), B(4), B(6), A(6)]),
		swizzle!(a, b, [A(1), B(1), B(3), A(3), A(5), B(5), B(7), A(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(1), B(1), A(7), B(7), A(6), B(6)]),
		swizzle!(a, b, [B(2), A(2), B(3), A(3), B(5), A(5), B(4), A(4)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), B(0), B(1), B(4), B(5), A(4), A(5)]),
		swizzle!(a, b, [A(2), A(3), B(2), B(3), B(6), B(7), A(6), A(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(2), B(2), B(4), A(4), B(6), A(6)]),
		swizzle!(a, b, [A(1), B(1), A(3), B(3), B(5), A(5), B(7), A(7)]),
	);

	let (a, b) = (
		swizzle!(a, b, [A(0), B(0), A(1), B(1), A(2), B(2), A(3), B(3)]),
		swizzle!(a, b, [B(4), A(4), B(5), A(5), B(6), A(6), B(7), A(7)]),
	);

	if a[7] <= b[7] {
		return (a, b.reverse());
	}

	let (a, b) = min_max!(a, b);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), A(2), A(3), B(0), B(1), B(2), B(3)]),
		swizzle!(a, b, [A(4), A(5), A(6), A(7), B(4), B(5), B(6), B(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), B(0), B(1), A(4), A(5), B(4), B(5)]),
		swizzle!(a, b, [A(2), A(3), B(2), B(3), A(6), A(7), B(6), B(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(2), B(2), A(4), B(4), A(6), B(6)]),
		swizzle!(a, b, [A(1), B(1), A(3), B(3), A(5), B(5), A(7), B(7)]),
	);

	(
		swizzle!(a, b, [A(0), B(0), A(1), B(1), A(2), B(2), A(3), B(3)]),
		swizzle!(a, b, [A(4), B(4), A(5), B(5), A(6), B(6), A(7), B(7)]),
	)
}

pub fn bitonic_sort_i32x16(a: i32x8, b: i32x8) -> (i32x8, i32x8) {
	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(3), A(4), A(7), B(0), B(3), B(4), B(7)]),
		swizzle!(a, b, [A(1), A(2), A(5), A(6), B(1), B(2), B(5), B(6)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(3), B(3), A(4), B(4), A(7), B(7)]),
		swizzle!(a, b, [B(1), A(1), B(2), A(2), B(5), A(5), B(6), A(6)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), B(2), A(2), A(4), B(4), B(6), A(6)]),
		swizzle!(a, b, [A(1), B(1), B(3), A(3), A(5), B(5), B(7), A(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(1), B(1), A(7), B(7), A(6), B(6)]),
		swizzle!(a, b, [B(2), A(2), B(3), A(3), B(5), A(5), B(4), A(4)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), B(0), B(1), B(4), B(5), A(4), A(5)]),
		swizzle!(a, b, [A(2), A(3), B(2), B(3), B(6), B(7), A(6), A(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(2), B(2), B(4), A(4), B(6), A(6)]),
		swizzle!(a, b, [A(1), B(1), A(3), B(3), B(5), A(5), B(7), A(7)]),
	);

	let (a, b) = (
		swizzle!(a, b, [A(0), B(0), A(1), B(1), A(2), B(2), A(3), B(3)]),
		swizzle!(a, b, [B(4), A(4), B(5), A(5), B(6), A(6), B(7), A(7)]),
	);

	if a[7] <= b[7] {
		return (a, b.reverse());
	}

	let (a, b) = min_max!(a, b);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), A(2), A(3), B(0), B(1), B(2), B(3)]),
		swizzle!(a, b, [A(4), A(5), A(6), A(7), B(4), B(5), B(6), B(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), B(0), B(1), A(4), A(5), B(4), B(5)]),
		swizzle!(a, b, [A(2), A(3), B(2), B(3), A(6), A(7), B(6), B(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(2), B(2), A(4), B(4), A(6), B(6)]),
		swizzle!(a, b, [A(1), B(1), A(3), B(3), A(5), B(5), A(7), B(7)]),
	);

	(
		swizzle!(a, b, [A(0), B(0), A(1), B(1), A(2), B(2), A(3), B(3)]),
		swizzle!(a, b, [A(4), B(4), A(5), B(5), A(6), B(6), A(7), B(7)]),
	)
}

pub fn dual_bitonic_sort_i32x16(a: i32x8, b: i32x8) -> (i32x8, i32x8) {
	let (a1, a2) = split_i32x8(a);
	let (b1, b2) = split_i32x8(b);

	let (a1, a2) = bitonic_sort_i32x8(a1, a2);
	let (b1, b2) = bitonic_sort_i32x8(b1, b2);

	let (a, b) = (
		join_i32x4(a1, a2),
		join_i32x4(b1, b2),
	);

	if a[7] <= b[0] {
		return (a, b);
	}

	let b = b.reverse();

	let (a, b) = min_max!(a, b);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), A(2), A(3), B(0), B(1), B(2), B(3)]),
		swizzle!(a, b, [A(4), A(5), A(6), A(7), B(4), B(5), B(6), B(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), B(0), B(1), A(4), A(5), B(4), B(5)]),
		swizzle!(a, b, [A(2), A(3), B(2), B(3), A(6), A(7), B(6), B(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(2), B(2), A(4), B(4), A(6), B(6)]),
		swizzle!(a, b, [A(1), B(1), A(3), B(3), A(5), B(5), A(7), B(7)]),
	);

	(
		swizzle!(a, b, [A(0), B(0), A(1), B(1), A(2), B(2), A(3), B(3)]),
		swizzle!(a, b, [A(4), B(4), A(5), B(5), A(6), B(6), A(7), B(7)]),
	)
}

pub fn bitonic_sort_i64x16(a: i64x8, b: i64x8) -> (i64x8, i64x8) {
	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(3), A(4), A(7), B(0), B(3), B(4), B(7)]),
		swizzle!(a, b, [A(1), A(2), A(5), A(6), B(1), B(2), B(5), B(6)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(3), B(3), A(4), B(4), A(7), B(7)]),
		swizzle!(a, b, [B(1), A(1), B(2), A(2), B(5), A(5), B(6), A(6)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), B(2), A(2), A(4), B(4), B(6), A(6)]),
		swizzle!(a, b, [A(1), B(1), B(3), A(3), A(5), B(5), B(7), A(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(1), B(1), A(7), B(7), A(6), B(6)]),
		swizzle!(a, b, [B(2), A(2), B(3), A(3), B(5), A(5), B(4), A(4)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), B(0), B(1), B(4), B(5), A(4), A(5)]),
		swizzle!(a, b, [A(2), A(3), B(2), B(3), B(6), B(7), A(6), A(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(2), B(2), B(4), A(4), B(6), A(6)]),
		swizzle!(a, b, [A(1), B(1), A(3), B(3), B(5), A(5), B(7), A(7)]),
	);

	let (a, b) = (
		swizzle!(a, b, [A(0), B(0), A(1), B(1), A(2), B(2), A(3), B(3)]),
		swizzle!(a, b, [B(4), A(4), B(5), A(5), B(6), A(6), B(7), A(7)]),
	);

	if a[7] <= b[7] {
		return (a, b.reverse());
	}

	let (a, b) = min_max!(a, b);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), A(2), A(3), B(0), B(1), B(2), B(3)]),
		swizzle!(a, b, [A(4), A(5), A(6), A(7), B(4), B(5), B(6), B(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), A(1), B(0), B(1), A(4), A(5), B(4), B(5)]),
		swizzle!(a, b, [A(2), A(3), B(2), B(3), A(6), A(7), B(6), B(7)]),
	);

	let (a, b) = min_max!(
		swizzle!(a, b, [A(0), B(0), A(2), B(2), A(4), B(4), A(6), B(6)]),
		swizzle!(a, b, [A(1), B(1), A(3), B(3), A(5), B(5), A(7), B(7)]),
	);

	(
		swizzle!(a, b, [A(0), B(0), A(1), B(1), A(2), B(2), A(3), B(3)]),
		swizzle!(a, b, [A(4), B(4), A(5), B(5), A(6), B(6), A(7), B(7)]),
	)
}
