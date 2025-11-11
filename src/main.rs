use perfos::{
    benchmark::{benchmark, Config},
    file::FilePolicy,
    runner::BenchmarkFunction,
    time,
};

use std::time::Instant;

////////////////////////////////////////

fn main() {
    let file_path = format!("example.benchmark");

    benchmark(Config {
        file_path: Some(file_path),
        default_file_policy: Some(FilePolicy::Rewrite),
        nb_buckets_around_avg: 5,
        nb_iterations: 50,
        functions: vec![
            BenchmarkFunction {
                name: "10".to_string(),
                f: Box::new(|| time!(|| fibonacci(10))),
            },
            BenchmarkFunction {
                name: "20".to_string(),
                f: Box::new(|| time!(|| fibonacci(20))),
            },
            BenchmarkFunction {
                name: "30".to_string(),
                f: Box::new(|| time!(|| fibonacci(30))),
            },
            BenchmarkFunction {
                name: "40".to_string(),
                f: Box::new(|| time!(|| fibonacci(40))),
            },
        ],
    });
}

////////////////////

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
