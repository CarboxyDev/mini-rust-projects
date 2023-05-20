#![allow(unused_parens)]
use std::time::SystemTime;

fn benchmark_function() {
    let mut _c: u32 = 0;
    for _i in 0..10000000 {
        _c += 1;
    }
}

fn do_benchmark(times: u64) -> f32 {
    let now = SystemTime::now();
    for _i in 0..times {
        benchmark_function();
    }

    let end = now
        .elapsed()
        .expect("Error: Unable to record the benchmark. The benchmark result may be inaccurate.");
    println!("Time taken: {}", end.as_secs_f32());
    return end.as_secs_f32();
}
fn main() {
    const BENCHMARK_COUNT: u64 = 10; // Number of times the benchmark should be timed
    const EACH_BENCHMARK_TIMES: u64 = 1; // Number of times the benchmark function should be ran in each timed benchmark
    let mut all_benchmarks: Vec<f32> = vec![];
    println!("[!] Doing the Benchmark {} times", BENCHMARK_COUNT);
    println!(
        "[!] Running the benchmark function in each Benchmark {} times",
        EACH_BENCHMARK_TIMES
    );
    for _i in 0..BENCHMARK_COUNT {
        let benchmark_time: f32 = do_benchmark(EACH_BENCHMARK_TIMES);
        all_benchmarks.push(benchmark_time);
    }

    if (BENCHMARK_COUNT > 1) {
        let avg_benchmark_time: f32 = all_benchmarks.iter().sum::<f32>() / (BENCHMARK_COUNT as f32);
        println!("---------------------------------------------------------------");
        println!("Average benchmark time: {}", avg_benchmark_time);
        println!("---------------------------------------------------------------");
    }
}
