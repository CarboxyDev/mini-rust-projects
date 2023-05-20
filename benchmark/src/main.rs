#![allow(unused_parens)]

use colored::Colorize;
use std::env;
use std::time::SystemTime;

fn benchmark_function() {
    let mut _c: u32 = 0;
    for _i in 0..10000000 {
        _c += 1;
    }
}

fn do_benchmark(benchmark_number: u64) -> f32 {
    let now = SystemTime::now();

    benchmark_function();

    let end = now
        .elapsed()
        .expect("Error: Unable to record the benchmark. The final result may be inaccurate due to this error.");

    println!(
        "[{}] Time taken: {}",
        format!("B{}", benchmark_number).blue(),
        end.as_secs_f32()
    );
    return end.as_secs_f32();
}
fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let benchmark_count: u64; // Number of times the benchmark should be timed
    println!("---------------------------------------------------------------");

    if (args.len() == 0) {
        println!("[!] Running on default setting");
        benchmark_count = 10;
    } else if (args[0].bytes().all(|c| c.is_ascii_digit())) {
        benchmark_count = args[0].parse::<u64>().unwrap()
    } else {
        println!("[!] You provided an incorrect argument. Running on default setting");
        benchmark_count = 10;
    }

    let mut all_benchmarks: Vec<f32> = vec![];
    println!("[!] Running the Benchmark {} times", benchmark_count);
    println!("---------------------------------------------------------------\n");
    for i in 1..benchmark_count + 1 {
        let benchmark_time: f32 = do_benchmark(i);
        all_benchmarks.push(benchmark_time);
    }

    if (benchmark_count > 1) {
        let avg_benchmark_time: f32 = all_benchmarks.iter().sum::<f32>() / (benchmark_count as f32);
        println!("\n---------------------------------------------------------------");
        println!("Average benchmark time: {}", avg_benchmark_time);
        println!("---------------------------------------------------------------");
    }
}
