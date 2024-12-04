use crate::constants::*;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering::Relaxed},
        Arc, Condvar, Mutex,
    },
    time::Duration,
};

const EMPTY_STRING: String = String::new();
const EMPTY_ARRAY: [String; 0] = [EMPTY_STRING; 0];
const TIMEOUT: Duration = Duration::from_millis(1);

pub struct BufferedQueue {
    is_done: AtomicBool,
    has_data: Condvar,
    data: Arc<Mutex<Vec<String>>>,
}

impl BufferedQueue {
    pub fn new() -> BufferedQueue {
        return BufferedQueue {
            is_done: AtomicBool::new(false),
            has_data: Condvar::new(),
            data: Arc::new(Mutex::new(Vec::with_capacity(QUEUE_BUFFER_SIZE))),
        };
    }

    pub fn add(&self, lines: &mut Vec<String>) {
        let mut data = self.data.lock().unwrap();
        self.has_data.notify_one();
        data.extend(lines.drain(..));
    }

    pub fn take(&self) -> Vec<String> {
        let mut data = self.data.lock().unwrap();
        if data.len() == 0 {
            let mut result = self.has_data.wait_timeout(data, TIMEOUT).unwrap();
            while result.1.timed_out() {
                // Wait for data until TIMEOUT occurs
                if self.is_done.load(Relaxed) {
                    // If the queue is done, return empty vector
                    return EMPTY_ARRAY.to_vec();
                }
                result = self.has_data.wait_timeout(result.0, TIMEOUT).unwrap();
            }
            data = result.0;
        }
        if THREAD_BUFFER_SIZE > data.len() {
            return data.drain(..).collect();
        }
        return data.drain(..THREAD_BUFFER_SIZE).collect();
    }

    pub fn done(&self) {
        self.is_done.store(true, Relaxed);
        self.has_data.notify_all();
    }
}

pub struct ParsedContent {
    pub max_monkey: u16,
    pub even_values: Vec<(u16, u16, u32)>,
    pub odd_values: Vec<(u16, u16, u32)>,
}

impl ParsedContent {
    pub fn new() -> ParsedContent {
        return ParsedContent {
            max_monkey: 0,
            even_values: Vec::with_capacity(QUEUE_BUFFER_SIZE / 2),
            odd_values: Vec::with_capacity(QUEUE_BUFFER_SIZE / 2),
        };
    }
}
