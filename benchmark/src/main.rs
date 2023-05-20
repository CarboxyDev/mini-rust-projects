#![allow(unused_parens)]
use std::time::SystemTime;

fn benchmark_function() {
    let mut _c: u32 = 0;
    for _i in 0..10000000 {
        _c += 1;
    }
}

fn do_benchmark() -> f32 {
    let now = SystemTime::now();

    benchmark_function();

    let end = now
        .elapsed()
        .expect("Error: Unable to record the benchmark. The final result may be inaccurate due to this error.");
    println!("Time taken: {}", end.as_secs_f32());
    return end.as_secs_f32();
}
fn main() {
    const BENCHMARK_COUNT: u64 = 10; // Number of times the benchmark should be timed
    let mut all_benchmarks: Vec<f32> = vec![];
    println!("---------------------------------------------------------------");
    println!("[!] Doing the Benchmark {} times", BENCHMARK_COUNT);
    println!("---------------------------------------------------------------\n");
    for _i in 0..BENCHMARK_COUNT {
        let benchmark_time: f32 = do_benchmark();
        all_benchmarks.push(benchmark_time);
    }

    if (BENCHMARK_COUNT > 1) {
        let avg_benchmark_time: f32 = all_benchmarks.iter().sum::<f32>() / (BENCHMARK_COUNT as f32);
        println!("\n---------------------------------------------------------------");
        println!("Average benchmark time: {}", avg_benchmark_time);
        println!("---------------------------------------------------------------");
    }
}
