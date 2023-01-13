use serde_json::Value;
use std::{thread, time};
use std::io::{stdout, stdin, Write};

pub fn get_name(get_name : &Value) -> String {
    let mut _t = get_name[0].as_str().unwrap();
    let name = input(_t);
    let alt_name = "Joseph";
    if name == "" {
        _t = get_name[1].as_str().unwrap();
        scroll_print(_t.replace("!alt_name", alt_name).as_str(), 50, true);
        // scroll_print(alt_name, 50, false);
        return alt_name.to_string();
    }
    return name;
}

fn input(msg : &str) -> String {
    let mut input = String::new();
    print!("{}", msg);
    stdout().flush().unwrap();
    stdin().read_line(&mut input).unwrap();
    
    return input.trim().to_string();
}

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