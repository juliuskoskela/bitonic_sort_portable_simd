#![feature(portable_simd)]

pub use std::simd::{
	simd_swizzle as swizzle,
	Which::{First as A, Second as B},
	*,
};

pub mod bitonic_sort;
pub use bitonic_sort::*;

#[macro_export]
macro_rules ! min_max {
	($a:expr, $b:expr $(,)*) => {
		(
			$a.simd_lt($b).select($a, $b),
			$a.simd_gt($b).select($a, $b),
		)
	};
}

const fn split_i32x8(a: i32x8) -> (i32x4, i32x4) {
	unsafe { std::mem::transmute(a) }
}

const fn join_i32x4(a: i32x4, b: i32x4) -> i32x8 {
	unsafe { std::mem::transmute((a, b)) }
}