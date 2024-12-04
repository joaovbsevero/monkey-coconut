use crate::constants::*;
use crate::data_structures::BufferedQueue;
use crate::utils::{
    get_best, get_hash, join_thread_vectors, parse_content, read_file, run_remaining_cycles,
    transfer_coconuts,
};
use std::collections::HashMap;
use std::thread;
use std::time::Instant;

pub fn parse_file(case_file: &String) -> (u32, (Vec<(u16, u32)>, Vec<(u16, u32)>)) {
    let buffered_queue = BufferedQueue::new();

    let (rounds, max_monkey, mut values) = thread::scope(|s| {
        let file_read_handle = s.spawn(|| {
            // ===== Read file content =====
            let start = Instant::now();
            let rounds = read_file(case_file, &buffered_queue);
            let duration = start.elapsed();
            println!("Read file time: {:?}", duration);
            // =============================

            return rounds;
        });

        let mut thread_handles = Vec::with_capacity(THREAD_NUM);

        // ======= Parse content =======
        let start = Instant::now();
        for _ in 0..THREAD_NUM {
            thread_handles.push(s.spawn(|| {
                return parse_content(&buffered_queue);
            }));
        }

        let rounds = file_read_handle.join().unwrap();

        let mut max_monkey: u16 = 0;
        let mut values = Vec::with_capacity(THREAD_NUM);
        for handle in thread_handles {
            let parsed_content = handle.join().unwrap();
            if max_monkey < parsed_content.max_monkey {
                max_monkey = parsed_content.max_monkey;
            }
            values.push((parsed_content.even_values, parsed_content.odd_values));
        }
        let duration = start.elapsed();
        println!("Parse content time: {:?}", duration);
        // =============================

        return (rounds, max_monkey + 1, values);
    });

    // ===== Join content =====
    let start = Instant::now();
    let joined_values = join_thread_vectors(max_monkey, &mut values);
    let duration = start.elapsed();
    println!("Join content time: {:?}", duration);
    // =============================

    // values.clear();

    return (rounds, joined_values);
}

pub fn execute_simulation(
    rounds: u32,
    even_values: &mut Vec<(u16, u32)>,
    odd_values: &mut Vec<(u16, u32)>,
) {
    let thread_closure = |name, values: &mut Vec<(u16, u32)>| {
        let mut states = HashMap::with_capacity(15);

        // ====== Run simulation =======
        let start = Instant::now();
        for cycle in 0..rounds {
            let hash = get_hash(values.clone());
            if let Some(cycle_start) = states.get(&hash) {
                run_remaining_cycles(cycle, cycle_start, rounds, values);
                break;
            }

            states.insert(hash, cycle);
            transfer_coconuts(values);
        }
        let simulation_duration = start.elapsed();

        println!("Simulation '{}' time: {:?}", name, simulation_duration);
        // =============================
    };

    thread::scope(|s| {
        let even_handler = s.spawn(|| {
            return thread_closure("even", even_values);
        });
        let odd_handler = s.spawn(|| {
            return thread_closure("odd", odd_values);
        });
        (even_handler.join().unwrap(), odd_handler.join().unwrap());
    });
}

pub fn display_best_monkey(even_results: Vec<(u16, u32)>, odd_results: Vec<(u16, u32)>) {
    // ========= Get best ==========
    let start = Instant::now();
    let (monkey_id, monkey_value) = get_best(even_results, odd_results);
    let duration = start.elapsed();
    println!("Get best time: {:?}", duration);
    // =============================

    println!(
        "\nBest monkey: {} with {} coconuts.",
        monkey_id, monkey_value
    );
}
