mod constants;
mod data_structures;
mod threading;
mod utils;

use std::env;
use std::time::Instant;

use threading::{display_best_monkey, execute_simulation, parse_file};

fn run(case_file: &String) {
    let (rounds, (mut even_values, mut odd_values)) = parse_file(case_file);
    execute_simulation(rounds, &mut even_values, &mut odd_values);
    display_best_monkey(even_values, odd_values);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let start = Instant::now();
    run(&file_path);
    let duration = start.elapsed();

    println!("\nTotal execution time: {:?}", duration);
}
