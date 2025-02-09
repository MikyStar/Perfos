use std::{sync::mpsc::sync_channel, time::Instant};

use crate::{
    console_ui::queue_msg,
    constants::{IMPORTING_PROJECT_NAME, IMPORTING_PROJECT_VERSION},
    file::{handle_file, write, FilePolicy},
    message_handler::handle_messages,
    runner::{execute_benchmarks, BenchmarkFunction, FuncThreadMessage},
};

////////////////////////////////////////

pub struct Config {
    pub nb_iterations: u16,
    pub functions: Vec<BenchmarkFunction>,
    pub nb_buckets_around_avg: u128,

    pub file_path: Option<String>,
    pub default_file_policy: Option<FilePolicy>,
}

////////////////////////////////////////

pub fn benchmark(config: Config) {
    let Config {
        nb_iterations,
        functions,
        nb_buckets_around_avg,
        file_path,
        default_file_policy,
    } = config;

    if let Some(ref path) = file_path {
        handle_file(path.to_string(), default_file_policy);
    }

    let txt: String;

    #[cfg(feature = "pkg_data")]
    {
        txt = format!("Benchmarking {IMPORTING_PROJECT_NAME}@v{IMPORTING_PROJECT_VERSION} with {nb_iterations} iterations\n");
    }

    #[cfg(not(feature = "pkg_data"))]
    {
        txt = format!("Benchmarking with {nb_iterations} iterations\n");
    }

    queue_msg(txt.clone());
    if let Some(ref path) = file_path.clone() {
        write(path.to_string(), vec![txt]);
    }

    let (tx, rx) = sync_channel::<FuncThreadMessage>(1);

    let f_names = functions.iter().map(|f| f.name.clone()).collect();

    let start = Instant::now();
    execute_benchmarks(tx, functions);
    handle_messages(rx, f_names, start, nb_buckets_around_avg, file_path);
}
