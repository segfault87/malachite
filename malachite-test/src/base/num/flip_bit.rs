use common::{m_run_benchmark, BenchmarkType, GenerationMode};
use inputs::base::{pairs_of_signed_and_u64_width_range, pairs_of_unsigned_and_u64_width_range};
use malachite_base::num::{PrimitiveSigned, PrimitiveUnsigned};

fn demo_unsigned_flip_bit<T: 'static + PrimitiveUnsigned>(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_unsigned_and_u64_width_range::<T>(gm).take(limit) {
        let n_old = n;
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

fn demo_signed_flip_bit<T: 'static + PrimitiveSigned>(gm: GenerationMode, limit: usize) {
    for (mut n, index) in pairs_of_signed_and_u64_width_range::<T>(gm).take(limit) {
        let n_old = n;
        n.flip_bit(index);
        println!("x := {}; x.flip_bit({}); x = {}", n_old, index, n);
    }
}

fn benchmark_unsigned_flip_bit<T: 'static + PrimitiveUnsigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.flip_bit(u64)", T::NAME),
        BenchmarkType::Ordinary,
        pairs_of_unsigned_and_u64_width_range::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "index",
        &[("malachite", &mut (|(mut n, index)| n.flip_bit(index)))],
    );
}

fn benchmark_signed_flip_bit<T: 'static + PrimitiveSigned>(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        &format!("{}.flip_bit(u64)", T::NAME),
        BenchmarkType::Ordinary,
        pairs_of_signed_and_u64_width_range::<T>(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, index)| index as usize),
        "index",
        &[("malachite", &mut (|(mut n, index)| n.flip_bit(index)))],
    );
}

macro_rules! unsigned {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_unsigned_flip_bit::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_unsigned_flip_bit::<$t>(gm, limit, file_name);
        }
    };
}

macro_rules! signed {
    ($t: ident, $demo_name: ident, $bench_name: ident) => {
        pub fn $demo_name(gm: GenerationMode, limit: usize) {
            demo_signed_flip_bit::<$t>(gm, limit);
        }

        pub fn $bench_name(gm: GenerationMode, limit: usize, file_name: &str) {
            benchmark_signed_flip_bit::<$t>(gm, limit, file_name);
        }
    };
}

unsigned!(u8, demo_u8_flip_bit, benchmark_u8_flip_bit);
unsigned!(u16, demo_u16_flip_bit, benchmark_u16_flip_bit);
unsigned!(u32, demo_u32_flip_bit, benchmark_u32_flip_bit);
unsigned!(u64, demo_u64_flip_bit, benchmark_u64_flip_bit);

signed!(i8, demo_i8_flip_bit, benchmark_i8_flip_bit);
signed!(i16, demo_i16_flip_bit, benchmark_i16_flip_bit);
signed!(i32, demo_i32_flip_bit, benchmark_i32_flip_bit);
signed!(i64, demo_i64_flip_bit, benchmark_i64_flip_bit);