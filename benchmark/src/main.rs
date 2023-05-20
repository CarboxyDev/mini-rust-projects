use std::time::SystemTime;

fn benchmark_function() {
    let mut _c: u32 = 0;
    for _i in 0..10000000 {
        _c += 1;
    }
}

fn do_benchmark(times: u64) {
    println!("[!] Running benchmark {} times", times);
    let now = SystemTime::now();
    for _i in 0..times {
        benchmark_function();
    }

    let end = now
        .elapsed()
        .expect("Error: Unable to record the benchmark");
    println!("Time taken: {}", end.as_secs_f32());
}
fn main() {
    const BENCHMARK_COUNT: u64 = 5; // Number of times the benchmark should be timed
    const EACH_BENCHMARK_TIMES: u64 = 1; // Number of times the benchmark function should be ran in each timed benchmark
    for _i in 0..BENCHMARK_COUNT {
        do_benchmark(EACH_BENCHMARK_TIMES);
    }
}
