use malachite_base::num::conversion::traits::{ToStringBase, WrappingFrom};
use malachite_base::strings::{ToBinaryString, ToLowerHexString, ToOctalString, ToUpperHexString};
use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::natural::conversion::string::to_string::BaseFmtWrapper;
use malachite_nz::test_util::bench::bucketers::{
    integer_bit_bucketer, pair_1_integer_bit_bucketer, triple_1_integer_bit_bucketer,
    triple_3_integer_bit_bucketer, triple_3_pair_1_integer_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    integer_gen, integer_gen_nrm, integer_unsigned_pair_gen_var_1,
    integer_unsigned_pair_gen_var_1_nrm, integer_unsigned_pair_gen_var_2,
    integer_unsigned_unsigned_triple_gen_var_1,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_to_string);
    register_demo!(runner, demo_integer_to_string_with_width);
    register_demo!(runner, demo_integer_to_binary_string);
    register_demo!(runner, demo_integer_to_binary_string_with_0b);
    register_demo!(runner, demo_integer_to_binary_string_with_width);
    register_demo!(runner, demo_integer_to_binary_string_with_0b_and_width);
    register_demo!(runner, demo_integer_to_octal_string);
    register_demo!(runner, demo_integer_to_octal_string_with_0o);
    register_demo!(runner, demo_integer_to_octal_string_with_width);
    register_demo!(runner, demo_integer_to_octal_string_with_0o_and_width);
    register_demo!(runner, demo_integer_to_lower_hex_string);
    register_demo!(runner, demo_integer_to_lower_hex_string_with_0x);
    register_demo!(runner, demo_integer_to_lower_hex_string_with_width);
    register_demo!(runner, demo_integer_to_lower_hex_string_with_0x_and_width);
    register_demo!(runner, demo_integer_to_upper_hex_string);
    register_demo!(runner, demo_integer_to_upper_hex_string_with_0x);
    register_demo!(runner, demo_integer_to_upper_hex_string_with_width);
    register_demo!(runner, demo_integer_to_upper_hex_string_with_0x_and_width);
    register_demo!(runner, demo_integer_to_string_base);
    register_demo!(runner, demo_integer_to_string_base_upper);
    register_demo!(runner, demo_integer_base_fmt_wrapper_fmt);
    register_demo!(runner, demo_integer_base_fmt_wrapper_fmt_upper);
    register_demo!(runner, demo_integer_base_fmt_wrapper_fmt_with_width);
    register_demo!(runner, demo_integer_base_fmt_wrapper_fmt_upper_with_width);
    register_bench!(runner, benchmark_integer_to_string_library_comparison);
    register_bench!(runner, benchmark_integer_to_string_algorithms);
    register_bench!(runner, benchmark_integer_to_string_with_width);
    register_bench!(
        runner,
        benchmark_integer_to_binary_string_library_comparison
    );
    register_bench!(runner, benchmark_integer_to_binary_string_algorithms);
    register_bench!(runner, benchmark_integer_to_binary_string_with_width);
    register_bench!(runner, benchmark_integer_to_octal_string_library_comparison);
    register_bench!(runner, benchmark_integer_to_octal_string_algorithms);
    register_bench!(runner, benchmark_integer_to_octal_string_with_width);
    register_bench!(
        runner,
        benchmark_integer_to_lower_hex_string_library_comparison
    );
    register_bench!(runner, benchmark_integer_to_lower_hex_string_algorithms);
    register_bench!(runner, benchmark_integer_to_lower_hex_string_with_width);
    register_bench!(
        runner,
        benchmark_integer_to_upper_hex_string_library_comparison
    );
    register_bench!(runner, benchmark_integer_to_upper_hex_string_with_width);
    register_bench!(runner, benchmark_integer_to_string_base_library_comparison);
    register_bench!(runner, benchmark_integer_to_string_base_algorithms);
    register_bench!(runner, benchmark_integer_to_string_base_upper_algorithms);
    register_bench!(runner, benchmark_integer_base_fmt_wrapper_fmt_with_width);
    register_bench!(
        runner,
        benchmark_integer_base_fmt_wrapper_fmt_upper_with_width
    );
}

fn demo_integer_to_string(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!("{}", n);
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_integer_to_string_with_width(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, width) in integer_unsigned_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "format!(\"{{:0{}}}\", {}) = {}",
            width,
            n,
            format!("{:0width$}", n, width = width)
        );
    }
}

fn demo_integer_to_binary_string(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!("{:b}", n);
    }
}

fn demo_integer_to_binary_string_with_0b(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!("{:#b}", n);
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_integer_to_binary_string_with_width(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, width) in integer_unsigned_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "format!(\"{{:0{}b}}\", {}) = {}",
            width,
            n,
            format!("{:0width$b}", n, width = width)
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_integer_to_binary_string_with_0b_and_width(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, width) in integer_unsigned_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "format!(\"{{:#0{}b}}\", {}) = {}",
            width,
            n,
            format!("{:#0width$b}", n, width = width)
        );
    }
}

fn demo_integer_to_octal_string(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!("{:o}", n);
    }
}

fn demo_integer_to_octal_string_with_0o(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!("{:#o}", n);
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_integer_to_octal_string_with_width(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, width) in integer_unsigned_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "format!(\"{{:0{}o}}\", {}) = {}",
            width,
            n,
            format!("{:0width$o}", n, width = width)
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_integer_to_octal_string_with_0o_and_width(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, width) in integer_unsigned_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "format!(\"{{:#0{}o}}\", {}) = {}",
            width,
            n,
            format!("{:#0width$o}", n, width = width)
        );
    }
}

fn demo_integer_to_lower_hex_string(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!("{:x}", n);
    }
}

fn demo_integer_to_lower_hex_string_with_0x(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!("{:#x}", n);
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_integer_to_lower_hex_string_with_width(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, width) in integer_unsigned_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "format!(\"{{:0{}x}}\", {}) = {}",
            width,
            n,
            format!("{:0width$x}", n, width = width)
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_integer_to_lower_hex_string_with_0x_and_width(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, width) in integer_unsigned_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "format!(\"{{:#0{}x}}\", {}) = {}",
            width,
            n,
            format!("{:#0width$x}", n, width = width)
        );
    }
}

fn demo_integer_to_upper_hex_string(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!("{:X}", n);
    }
}

fn demo_integer_to_upper_hex_string_with_0x(gm: GenMode, config: GenConfig, limit: usize) {
    for n in integer_gen().get(gm, &config).take(limit) {
        println!("{:#X}", n);
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_integer_to_upper_hex_string_with_width(gm: GenMode, config: GenConfig, limit: usize) {
    for (n, width) in integer_unsigned_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "format!(\"{{:0{}X}}\", {}) = {}",
            width,
            n,
            format!("{:0width$X}", n, width = width)
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_integer_to_upper_hex_string_with_0x_and_width(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (n, width) in integer_unsigned_pair_gen_var_2()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "format!(\"{{:#0{}X}}\", {}) = {}",
            width,
            n,
            format!("{:#0width$X}", n, width = width)
        );
    }
}

fn demo_integer_to_string_base(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, base) in integer_unsigned_pair_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "({}).to_string_base({}) = {}",
            x,
            base,
            x.to_string_base(base)
        );
    }
}

fn demo_integer_to_string_base_upper(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, base) in integer_unsigned_pair_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "({}).to_string_base_upper({}) = {}",
            x,
            base,
            x.to_string_base_upper(base)
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_integer_base_fmt_wrapper_fmt(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, base) in integer_unsigned_pair_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "format!(\"{{}}\", BaseFmtWrapper::new({}, {})) = {}",
            x,
            base,
            format!("{}", BaseFmtWrapper::new(&x, base))
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_integer_base_fmt_wrapper_fmt_upper(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, base) in integer_unsigned_pair_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "format!(\"{{:#}}\", BaseFmtWrapper::new({}, {})) = {}",
            x,
            base,
            format!("{:#}", BaseFmtWrapper::new(&x, base))
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_integer_base_fmt_wrapper_fmt_with_width(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, base, width) in integer_unsigned_unsigned_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "format!(\"{{:0{}}}\", BaseFmtWrapper::new({}, {})) = {}",
            width,
            x,
            base,
            format!("{:0width$}", BaseFmtWrapper::new(&x, base), width = width)
        );
    }
}

#[allow(clippy::format_in_format_args)]
fn demo_integer_base_fmt_wrapper_fmt_upper_with_width(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for (x, base, width) in integer_unsigned_unsigned_triple_gen_var_1()
        .get(gm, &config)
        .take(limit)
    {
        println!(
            "format!(\"{{:#0{}}}\", BaseFmtWrapper::new({}, {})) = {}",
            width,
            x,
            base,
            format!("{:#0width$}", BaseFmtWrapper::new(&x, base), width = width)
        );
    }
}

fn benchmark_integer_to_string_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_string()",
        BenchmarkType::LibraryComparison,
        integer_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| no_out!(x.to_string())),
            ("num", &mut |(x, _, _)| no_out!(x.to_string())),
            ("rug", &mut |(_, x, _)| no_out!(x.to_string())),
        ],
    );
}

fn benchmark_integer_to_string_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_string()",
        BenchmarkType::Algorithms,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.to_string())),
            ("to_string_base", &mut |x| no_out!(x.to_string_base(10))),
        ],
    );
}

fn benchmark_integer_to_string_with_width(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "format!(\"{{:0usize}}\", Integer)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, width)| {
            no_out!(format!("{:0width$}", x, width = width))
        })],
    );
}

fn benchmark_integer_to_binary_string_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_binary_string()",
        BenchmarkType::LibraryComparison,
        integer_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| no_out!(x.to_binary_string())),
            ("num", &mut |(x, _, _)| no_out!(x.to_binary_string())),
            ("rug", &mut |(_, x, _)| no_out!(x.to_binary_string())),
        ],
    );
}

fn benchmark_integer_to_binary_string_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_binary_string()",
        BenchmarkType::Algorithms,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.to_binary_string())),
            ("to_string_base", &mut |x| no_out!(x.to_string_base(2))),
        ],
    );
}

fn benchmark_integer_to_binary_string_with_width(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "format!(\"{{:0usizeb}}\", Integer)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, width)| {
            no_out!(format!("{:0width$b}", x, width = width))
        })],
    );
}

fn benchmark_integer_to_octal_string_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_octal_string()",
        BenchmarkType::LibraryComparison,
        integer_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| no_out!(x.to_octal_string())),
            ("num", &mut |(x, _, _)| no_out!(x.to_octal_string())),
            ("rug", &mut |(_, x, _)| no_out!(x.to_octal_string())),
        ],
    );
}

fn benchmark_integer_to_octal_string_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_octal_string()",
        BenchmarkType::Algorithms,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.to_octal_string())),
            ("to_string_base", &mut |x| no_out!(x.to_string_base(8))),
        ],
    );
}

fn benchmark_integer_to_octal_string_with_width(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "format!(\"{{:0usizeo}}\", Integer)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, width)| {
            no_out!(format!("{:0width$o}", x, width = width))
        })],
    );
}

fn benchmark_integer_to_lower_hex_string_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_lower_hex_string()",
        BenchmarkType::LibraryComparison,
        integer_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| {
                no_out!(x.to_lower_hex_string())
            }),
            ("num", &mut |(x, _, _)| no_out!(x.to_lower_hex_string())),
            ("rug", &mut |(_, x, _)| no_out!(x.to_lower_hex_string())),
        ],
    );
}

fn benchmark_integer_to_lower_hex_string_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_lower_hex_string()",
        BenchmarkType::Algorithms,
        integer_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &integer_bit_bucketer("x"),
        &mut [
            ("default", &mut |x| no_out!(x.to_lower_hex_string())),
            ("to_string_base", &mut |x| no_out!(x.to_string_base(16))),
        ],
    );
}

fn benchmark_integer_to_lower_hex_string_with_width(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "format!(\"{{:0usizex}}\", Integer)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, width)| {
            no_out!(format!("{:0width$x}", x, width = width))
        })],
    );
}

fn benchmark_integer_to_upper_hex_string_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_upper_hex_string()",
        BenchmarkType::LibraryComparison,
        integer_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, x)| {
                no_out!(x.to_upper_hex_string())
            }),
            ("num", &mut |(x, _, _)| no_out!(x.to_upper_hex_string())),
            ("rug", &mut |(_, x, _)| no_out!(x.to_upper_hex_string())),
        ],
    );
}

fn benchmark_integer_to_upper_hex_string_with_width(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "format!(\"{{:0usizeX}}\", Integer)",
        BenchmarkType::Single,
        integer_unsigned_pair_gen_var_2().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, width)| {
            no_out!(format!("{:0width$X}", x, width = width))
        })],
    );
}

fn benchmark_integer_to_string_base_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_string_base(u64)",
        BenchmarkType::LibraryComparison,
        integer_unsigned_pair_gen_var_1_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_1_integer_bit_bucketer("x"),
        &mut [
            ("Malachite", &mut |(_, _, (x, base))| {
                no_out!(x.to_string_base(base))
            }),
            ("num", &mut |((x, base), _, _)| {
                no_out!(x.to_str_radix(u32::wrapping_from(base)))
            }),
            ("rug", &mut |(_, (x, base), _)| {
                no_out!(x.to_string_radix(i32::wrapping_from(base)))
            }),
        ],
    );
}

fn benchmark_integer_to_string_base_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_string_base(u64)",
        BenchmarkType::Algorithms,
        integer_unsigned_pair_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            (
                "to_string",
                &mut |(x, base)| no_out!(x.to_string_base(base)),
            ),
            ("using fmt", &mut |(x, base)| {
                no_out!(format!("{}", BaseFmtWrapper::new(&x, base)))
            }),
        ],
    );
}

fn benchmark_integer_to_string_base_upper_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.to_string_base_upper(u64)",
        BenchmarkType::Algorithms,
        integer_unsigned_pair_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &pair_1_integer_bit_bucketer("x"),
        &mut [
            ("to_string", &mut |(x, base)| {
                no_out!(x.to_string_base_upper(base))
            }),
            ("using fmt", &mut |(x, base)| {
                no_out!(format!("{:#}", BaseFmtWrapper::new(&x, base)))
            }),
        ],
    );
}

fn benchmark_integer_base_fmt_wrapper_fmt_with_width(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "format!(\"{{:0usize}}\", BaseFmtWrapper::new(Integer, u64))",
        BenchmarkType::Single,
        integer_unsigned_unsigned_triple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, base, width)| {
            no_out!(format!(
                "{:0width$}",
                BaseFmtWrapper::new(&x, base),
                width = width
            ))
        })],
    );
}

fn benchmark_integer_base_fmt_wrapper_fmt_upper_with_width(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "format!(\"{{:#0usize}}\", BaseFmtWrapper::new(Integer, u64))",
        BenchmarkType::Single,
        integer_unsigned_unsigned_triple_gen_var_1().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_1_integer_bit_bucketer("x"),
        &mut [("Malachite", &mut |(x, base, width)| {
            no_out!(format!(
                "{:#0width$}",
                BaseFmtWrapper::new(&x, base),
                width = width
            ))
        })],
    );
}
