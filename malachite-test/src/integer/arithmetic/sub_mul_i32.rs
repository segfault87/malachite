use common::GenerationMode;
use inputs::integer::triples_of_integer_integer_and_signed;
use malachite_base::num::SignificantBits;
use malachite_base::num::{SubMul, SubMulAssign};
use malachite_nz::integer::Integer;
use rust_wheels::benchmarks::{BenchmarkOptions1, BenchmarkOptions2, BenchmarkOptions4,
                              benchmark_1, benchmark_2, benchmark_4};
use std::cmp::max;

pub fn demo_integer_sub_mul_assign_i32(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integer_integer_and_signed::<i32>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        a.sub_mul_assign(b, c);
        println!(
            "a := {}; x.sub_mul_assign({}, {}); x = {}",
            a_old, b_old, c, a
        );
    }
}

pub fn demo_integer_sub_mul_assign_i32_ref(gm: GenerationMode, limit: usize) {
    for (mut a, b, c) in triples_of_integer_integer_and_signed::<i32>(gm).take(limit) {
        let a_old = a.clone();
        a.sub_mul_assign(&b, c);
        println!("a := {}; x.sub_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

pub fn demo_integer_sub_mul_i32(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_signed::<i32>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("{}.sub_mul({}, {}) = {}", a_old, b_old, c, a.sub_mul(b, c));
    }
}

pub fn demo_integer_sub_mul_i32_val_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_signed::<i32>(gm).take(limit) {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, {}) = {}", a_old, b, c, a.sub_mul(&b, c));
    }
}

pub fn demo_integer_sub_mul_i32_ref_val(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_signed::<i32>(gm).take(limit) {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "(&{}).sub_mul({}, {}) = {}",
            a_old,
            b_old,
            c,
            (&a).sub_mul(b, c)
        );
    }
}

pub fn demo_integer_sub_mul_i32_ref_ref(gm: GenerationMode, limit: usize) {
    for (a, b, c) in triples_of_integer_integer_and_signed::<i32>(gm).take(limit) {
        let a_old = a.clone();
        println!(
            "(&{}).sub_mul(&{}, {}) = {}",
            a_old,
            b,
            c,
            (&a).sub_mul(&b, c)
        );
    }
}

pub fn benchmark_integer_sub_mul_assign_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.sub_mul_assign(Integer, i32)",
        gm.name()
    );
    benchmark_1(BenchmarkOptions1 {
        xs: triples_of_integer_integer_and_signed::<i32>(gm),
        function_f: &mut (|(mut a, b, c): (Integer, Integer, i32)| a.sub_mul_assign(b, c)),
        x_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        title: "Integer.sub_mul_assign(Integer, i32)",
        x_axis_label: "max(a.significant_bits(), b.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_assign_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.sub_mul_assign(Integer, i32) evaluation strategy",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_integer_integer_and_signed::<i32>(gm),
        function_f: &mut (|(mut a, b, c): (Integer, Integer, i32)| a.sub_mul_assign(b, c)),
        function_g: &mut (|(mut a, b, c): (Integer, Integer, i32)| a.sub_mul_assign(&b, c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Integer.sub_mul_assign(Integer, i32)",
        g_name: "Integer.sub_mul_assign(&Integer, i32)",
        title: "Integer.sub_mul_assign(Integer, i32) evaluation strategy",
        x_axis_label: "max(a.significant_bits(), b.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_assign_i32_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.sub_mul_assign(Integer, i32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_integer_integer_and_signed::<i32>(gm),
        function_f: &mut (|(mut a, b, c): (Integer, Integer, i32)| a.sub_mul_assign(b, c)),
        function_g: &mut (|(mut a, b, c): (Integer, Integer, i32)| a -= b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Integer.sub_mul_assign(Integer, i32)",
        g_name: "Integer -= Integer * i32",
        title: "Integer.sub_mul_assign(Integer, i32) algorithms",
        x_axis_label: "max(a.significant_bits(), b.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_assign_i32_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.sub_mul_assign(&Integer, i32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_integer_integer_and_signed::<i32>(gm),
        function_f: &mut (|(mut a, b, c): (Integer, Integer, i32)| a.sub_mul_assign(&b, c)),
        function_g: &mut (|(mut a, b, c): (Integer, Integer, i32)| a -= &b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Integer.sub_mul_assign(&Integer, i32)",
        g_name: "Integer -= &Integer * i32",
        title: "Integer.sub_mul_assign(&Integer, i32) algorithms",
        x_axis_label: "max(a.significant_bits(), b.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_i32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.sub_mul(Integer, i32)", gm.name());
    benchmark_1(BenchmarkOptions1 {
        xs: triples_of_integer_integer_and_signed::<i32>(gm),
        function_f: &mut (|(a, b, c): (Integer, Integer, i32)| a.sub_mul(b, c)),
        x_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "malachite",
        title: "Integer.sub_mul(Integer, i32)",
        x_axis_label: "max(a.significant_bits(), b.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_i32_evaluation_strategy(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.sub_mul(Integer, i32) evaluation strategy",
        gm.name()
    );
    benchmark_4(BenchmarkOptions4 {
        xs: triples_of_integer_integer_and_signed::<i32>(gm),
        function_f: &mut (|(a, b, c): (Integer, Integer, i32)| a.sub_mul(b, c)),
        function_g: &mut (|(a, b, c): (Integer, Integer, i32)| a.sub_mul(&b, c)),
        function_h: &mut (|(a, b, c): (Integer, Integer, i32)| (&a).sub_mul(b, c)),
        function_i: &mut (|(a, b, c): (Integer, Integer, i32)| (&a).sub_mul(&b, c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        z_cons: &(|t| t.clone()),
        w_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Integer.sub_mul(Integer, i32)",
        g_name: "Integer.sub_mul(&Integer, i32)",
        h_name: "(&Integer).sub_mul(Integer, i32)",
        i_name: "(&Integer).sub_mul(&Integer, i32)",
        title: "Integer.sub_mul(&Integer, i32) evaluation strategy",
        x_axis_label: "max(a.significant_bits(), b.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_i32_algorithms(gm: GenerationMode, limit: usize, file_name: &str) {
    println!(
        "benchmarking {} Integer.sub_mul(Integer, i32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_integer_integer_and_signed::<i32>(gm),
        function_f: &mut (|(a, b, c): (Integer, Integer, i32)| a.sub_mul(b, c)),
        function_g: &mut (|(a, b, c): (Integer, Integer, i32)| a - b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Integer.sub_mul(Integer, i32)",
        g_name: "Integer - Integer * i32",
        title: "Integer.sub_mul(Integer, i32) algorithms",
        x_axis_label: "max(a.significant_bits(), b.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_i32_val_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} Integer.sub_mul(&Integer, i32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_integer_integer_and_signed::<i32>(gm),
        function_f: &mut (|(a, b, c): (Integer, Integer, i32)| a.sub_mul(&b, c)),
        function_g: &mut (|(a, b, c): (Integer, Integer, i32)| a - &b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "Integer.sub_mul(&Integer, i32)",
        g_name: "Integer - &Integer * i32",
        title: "Integer.sub_mul(&Integer, i32) algorithms",
        x_axis_label: "max(a.significant_bits(), b.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_i32_ref_val_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} (&Integer).sub_mul(Integer, i32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_integer_integer_and_signed::<i32>(gm),
        function_f: &mut (|(a, b, c): (Integer, Integer, i32)| (&a).sub_mul(b, c)),
        function_g: &mut (|(a, b, c): (Integer, Integer, i32)| &a - b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "(&Integer).sub_mul(Integer, i32)",
        g_name: "(&Integer) - Integer * i32",
        title: "(&Integer).sub_mul(Integer, i32) algorithms",
        x_axis_label: "max(a.significant_bits(), b.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_sub_mul_i32_ref_ref_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    println!(
        "benchmarking {} (&Integer).sub_mul(&Integer, i32) algorithms",
        gm.name()
    );
    benchmark_2(BenchmarkOptions2 {
        xs: triples_of_integer_integer_and_signed::<i32>(gm),
        function_f: &mut (|(a, b, c): (Integer, Integer, i32)| (&a).sub_mul(&b, c)),
        function_g: &mut (|(a, b, c): (Integer, Integer, i32)| &a - &b * c),
        x_cons: &(|t| t.clone()),
        y_cons: &(|t| t.clone()),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit,
        f_name: "(&Integer).sub_mul(&Integer, i32)",
        g_name: "(&Integer) - &Integer * i32",
        title: "(&Integer).sub_mul(&Integer, i32) algorithms",
        x_axis_label: "max(a.significant_bits(), b.significant_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}