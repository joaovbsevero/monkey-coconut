use crate::constants::*;
use crate::data_structures::{BufferedQueue, ParsedContent};
use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{prelude::*, BufReader};

pub fn read_file(file_path: &String, buffered_queue: &BufferedQueue) -> u32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut lines_iter = reader.lines();

    let rounds_line = lines_iter.next().unwrap().unwrap();
    let rounds: u32 = rounds_line.split(" ").nth(1).unwrap().parse().unwrap();

    let mut stop = false;
    let mut current_buffer = Vec::with_capacity(THREAD_BUFFER_SIZE);
    while !stop {
        for _ in 0..THREAD_BUFFER_SIZE {
            let line = lines_iter.next();
            if line.is_none() {
                stop = true;
                break;
            }
            // If line is empty, we have reached the end of the file
            let content = line.unwrap().unwrap();
            if content.is_empty() {
                stop = true;
                break;
            }

            current_buffer.push(content);
        }
        buffered_queue.add(&mut current_buffer);
    }

    buffered_queue.done();
    return rounds;
}

pub fn parse_content(buffered_queue: &BufferedQueue) -> ParsedContent {
    let mut parsed_content = ParsedContent::new();

    let mut even_count: u32;
    let mut odd_count: u32;
    loop {
        let lines = buffered_queue.take();
        if lines.len() == 0 {
            break;
        }

        let lines_iter = lines.iter().peekable();
        for line in lines_iter {
            let mut splitted_line = line.trim().split(" ");

            // Get current monkey number
            let monkey: u16 = splitted_line.nth(1).unwrap().parse().unwrap();
            if monkey > parsed_content.max_monkey {
                parsed_content.max_monkey = monkey;
            }

            // Get target monkey number
            let even_monkey: u16 = splitted_line.nth(2).unwrap().parse().unwrap();
            let odd_monkey: u16 = splitted_line.nth(2).unwrap().parse().unwrap();

            // Get amount of values
            even_count = 0;
            odd_count = 0;
            for value in splitted_line.skip(3) {
                let value = value.chars().last().unwrap().to_digit(10).unwrap() as u8;
                even_count += (value % 2 == 0) as u32;
                odd_count += (value % 2) as u32;
            }

            parsed_content
                .even_values
                .push((monkey, even_monkey, even_count));
            parsed_content
                .odd_values
                .push((monkey, odd_monkey, odd_count));
        }
    }

    parsed_content.even_values.shrink_to_fit();
    parsed_content.odd_values.shrink_to_fit();

    return parsed_content;
}

pub fn join_thread_vectors(
    max_monkey: u16,
    values: &Vec<(Vec<(u16, u16, u32)>, Vec<(u16, u16, u32)>)>,
) -> (Vec<(u16, u32)>, Vec<(u16, u32)>) {
    let mut joined_even_values: Vec<(u16, u32)> = Vec::with_capacity(max_monkey as usize);
    let mut joined_odd_values: Vec<(u16, u32)> = Vec::with_capacity(max_monkey as usize);

    unsafe {
        joined_even_values.set_len(max_monkey as usize);
        joined_odd_values.set_len(max_monkey as usize);
    }

    for (even_values, odd_values) in values {
        for (monkey, target, count) in even_values {
            joined_even_values[*monkey as usize] = (*target, *count);
        }
        for (monkey, target, count) in odd_values {
            joined_odd_values[*monkey as usize] = (*target, *count);
        }
    }

    return (joined_even_values, joined_odd_values);
}

pub fn transfer_coconuts(values: &mut Vec<(u16, u32)>) {
    for source_idx in 0..values.len() {
        let source_mapping = values[source_idx];
        let target_idx = source_mapping.0 as usize;

        values[target_idx].1 += source_mapping.1;
        values[source_idx].1 = 0;
    }
}

pub fn get_best(even_results: Vec<(u16, u32)>, odd_results: Vec<(u16, u32)>) -> (i32, u32) {
    let mut best_monkey_id: i32 = -1;
    let mut best_monkey_value: u32 = 0;

    for (idx, (a, b)) in even_results.iter().zip(odd_results.iter()).enumerate() {
        let monkey_value = a.1 + b.1;
        if monkey_value > best_monkey_value {
            best_monkey_value = monkey_value;
            best_monkey_id = idx as i32;
        }
    }

    return (best_monkey_id, best_monkey_value);
}

pub fn get_hash(values: Vec<(u16, u32)>) -> u64 {
    let mut hasher = DefaultHasher::new();
    values.hash(&mut hasher);
    return hasher.finish();
}

pub fn run_remaining_cycles(
    cycle: u32,
    cycle_start: &u32,
    rounds: u32,
    values: &mut Vec<(u16, u32)>,
) {
    let cycle_length = cycle - cycle_start;
    let pre_cycle_length = cycle_start;
    let remaining_iterations = (rounds - cycle) % (cycle_length + pre_cycle_length);
    let total_iterations = cycle_start + remaining_iterations;

    for _ in 0..total_iterations {
        transfer_coconuts(values);
    }
}
