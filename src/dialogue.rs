use std::fs::File;
use std::io::BufReader;
use serde_json::Value;

pub fn file_read() -> Value {
    // this will open dialogue.json and read it into serde json
    let file = File::open("./src/dialogue.json").expect("Unable to open file");
    let reader = BufReader::new(file);
    let json: Value = serde_json::from_reader(reader).expect("Unable to parse json");
    let v: Value = serde_json::from_str(&json.to_string()).unwrap();

    // return the json
    v
}