#![feature(portable_simd)]
use bitonic_sort_portable_simd::*;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;

fn vector_of_f32x8(size: usize) -> Vec<[f32; 8]> {
	let mut rng = rand::thread_rng();
	let mut vec = Vec::with_capacity(size);
	for _ in 0..size {
		let mut arr = [0.0; 8];
		for i in 0..8 {
			arr[i] = rng.gen();
		}
		vec.push(arr);
	}
	vec
}

fn vector_of_f32x16(size: usize) -> Vec<[f32; 16]> {
	let mut rng = rand::thread_rng();
	let mut vec = Vec::with_capacity(size);
	for _ in 0..size {
		let mut arr = [0.0; 16];
		for i in 0..16 {
			arr[i] = rng.gen();
		}
		vec.push(arr);
	}
	vec
}

fn vector_of_i32x8(size: usize) -> Vec<[i32; 8]> {
	let mut rng = rand::thread_rng();
	let mut vec = Vec::with_capacity(size);
	for _ in 0..size {
		let mut arr = [0; 8];
		for i in 0..8 {
			arr[i] = rng.gen();
		}
		vec.push(arr);
	}
	vec
}

fn vector_of_i32x16(size: usize) -> Vec<[i32; 16]> {
	let mut rng = rand::thread_rng();
	let mut vec = Vec::with_capacity(size);
	for _ in 0..size {
		let mut arr = [0; 16];
		for i in 0..16 {
			arr[i] = rng.gen();
		}
		vec.push(arr);
	}
	vec
}


fn is_fsorted(v: &[f32]) -> bool {
	for i in 1..v.len() {
		if v[i - 1] > v[i] {
			return false;
		}
	}
	true
}

fn is_isorted(v: &[i32]) -> bool {
	for i in 1..v.len() {
		if v[i - 1] > v[i] {
			return false;
		}
	}
	true
}

fn bench_std_sort(c: &mut Criterion) {

	let mut group = c.benchmark_group("STD");

	let size = 10_000;

	group.bench_with_input(BenchmarkId::new("f32", 8), &0, |b, _| {
		let mut v = vector_of_f32x8(size);
		b.iter(|| {
			black_box({
				for i in 0..size {
					v[i].sort_by(|a, b| a.partial_cmp(b).unwrap());
					assert!(is_fsorted(&v[i]));
				}
			});
		});
	});

	group.bench_with_input(BenchmarkId::new("i32", 8), &0, |b, _| {
		let mut v = vector_of_i32x8(size);
		b.iter(|| {
			black_box({
				for i in 0..size {
					v[i].sort();
					assert!(is_isorted(&v[i]));
				}
			});
		});
	});

	group.bench_with_input(BenchmarkId::new("f32", 16), &0, |b, _| {
		let mut v = vector_of_f32x16(size);
		b.iter(|| {
			black_box({
				for i in 0..size {
					v[i].sort_by(|a, b| a.partial_cmp(b).unwrap());
					assert!(is_fsorted(&v[i]));
				}
			});
		});
	});

	group.bench_with_input(BenchmarkId::new("i32", 16), &0, |b, _| {
		let mut v = vector_of_i32x16(size);
		b.iter(|| {
			black_box({
				for i in 0..size {
					v[i].sort();
					assert!(is_isorted(&v[i]));
				}
			});
		});
	});

	group.finish();
}

fn bench_simd_bitonic_sort(c: &mut Criterion) {

	let mut group = c.benchmark_group("BITONIC");

	let size = 10_000;

	group.bench_with_input(BenchmarkId::new("f32", 8), &0, |b, _| {
		let mut v = vector_of_f32x8(size);
		b.iter(|| {
			black_box({
				for i in 0..size {
					let arr = v[i];
					let (a, b) = arr.split_at(4);
					let (a, b) = (f32x4::from_slice(a), f32x4::from_slice(b));
					let (a, b) = bitonic_sort_f32x8(a, b);
					let a = a.as_array();
					let b = b.as_array();
					v[i][..4].copy_from_slice(a);
					v[i][4..].copy_from_slice(b);
					assert!(is_fsorted(&v[i]));
				}
			});
		});
	});

	group.bench_with_input(BenchmarkId::new("i32", 8), &0, |b, _| {
		let mut v = vector_of_i32x8(size);
		b.iter(|| {
			black_box({
				for i in 0..size {
					let arr = v[i];
					let (a, b) = arr.split_at(4);
					let (a, b) = (i32x4::from_slice(a), i32x4::from_slice(b));
					let (a, b) = bitonic_sort_i32x8(a, b);
					let a = a.as_array();
					let b = b.as_array();
					v[i][..4].copy_from_slice(a);
					v[i][4..].copy_from_slice(b);
					assert!(is_isorted(&v[i]));
				}
			});
		});
	});

	group.bench_with_input(BenchmarkId::new("f32", 16), &0, |b, _| {
		let mut v = vector_of_f32x16(size);
		b.iter(|| {
			black_box({
				for i in 0..size {
					let arr = v[i];
					let (a, b) = arr.split_at(8);
					let (a, b) = (f32x8::from_slice(a), f32x8::from_slice(b));
					let (a, b) = bitonic_sort_f32x16(a, b);
					let a = a.as_array();
					let b = b.as_array();
					v[i][..8].copy_from_slice(a);
					v[i][8..].copy_from_slice(b);
					assert!(is_fsorted(&v[i]));
				}
			});
		});
	});

	group.bench_with_input(BenchmarkId::new("i32", 16), &0, |b, _| {
		let mut v = vector_of_i32x16(size);
		b.iter(|| {
			black_box({
				for i in 0..size {
					let arr = v[i];
					let (a, b) = arr.split_at(8);
					let (a, b) = (i32x8::from_slice(a), i32x8::from_slice(b));
					let (a, b) = bitonic_sort_i32x16(a, b);
					let a = a.as_array();
					let b = b.as_array();
					v[i][..8].copy_from_slice(a);
					v[i][8..].copy_from_slice(b);
					assert!(is_isorted(&v[i]));
				}
			});
		});
	});

	group.finish();
}

criterion_group!(
    benches,
	bench_std_sort,
	bench_simd_bitonic_sort,
);
criterion_main!(benches);
