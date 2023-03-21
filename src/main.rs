use rand::Rng;
use std::env;
use std::time::Instant;
use tokio::task;
use sysinfo::{ProcessExt, System, SystemExt};

const MB_1: usize = 1_000_000;
//const MB_1: usize = 1;

// Synchronous function
#[inline(never)]
fn allocate_sync() -> u8 {
    let mut rng = rand::thread_rng();
    let mut data = vec![0u8; MB_1];

    for elem in data.iter_mut() {
        *elem = rng.gen();
    }

    *data.iter().max().unwrap()
}

// Asynchronous function
#[inline(never)]
async fn allocate_async() -> u8 {
    let mut rng = rand::thread_rng();
    let mut data = vec![0u8; MB_1];

    for elem in data.iter_mut() {
        *elem = rng.gen();
    }

    *data.iter().max().unwrap()
}

fn get_memory_usage() -> u64 {
    let mut system = System::new_all();
    system.refresh_all();

    let current_pid = sysinfo::get_current_pid().unwrap();
    let current_process = system.process(current_pid).unwrap();
    current_process.memory()
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} [sync|async] [iterations]", args[0]);
        return;
    }

    let mode = &args[1];
    let iterations: usize = args[2].parse().unwrap_or(10);

    let start = Instant::now();
    let initial_memory_usage = get_memory_usage() / 1024;

    match mode.as_str() {
        "sync" => {
            for _ in 0..iterations {
                allocate_sync();
            }
        }
        "async" => {
            let mut tasks = Vec::new();
            for _ in 0..iterations {
                tasks.push(task::spawn(allocate_async()));
            }
            for t in tasks {
                t.await.unwrap();
            }
        }
        _ => {
            println!("Invalid mode. Please use 'sync' or 'async'");
            return;
        }
    }

    let elapsed = start.elapsed();
    let final_memory_usage = get_memory_usage() / 1024;
    let memory_difference = final_memory_usage as i64 - initial_memory_usage as i64;

    println!(
        "Completed {} iterations in {:?} in {} mode",
        iterations, elapsed, mode
    );
    println!(
        "Initial memory usage: {} KB\nFinal memory usage: {} KB\nDifference: {} KB",
        initial_memory_usage, final_memory_usage, memory_difference
    );
}
