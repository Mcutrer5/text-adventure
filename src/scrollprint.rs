use std::{thread, time};
use std::io::{stdout, Write};

fn scroll_print(text : &str, wait_time: u64, new_line: bool) {
    for c in text.chars() {
        let mut lock = stdout().lock();
        write!(lock, "{}", c).unwrap();
        thread::sleep(time::Duration::from_millis(wait_time));
        lock.flush().unwrap();
    }
    if new_line {
        println!();
    }
}