use serde_json::json;
use serde_json::value::Value;
use serde_json::Map;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read};

pub fn read_file(file_name: &str) -> Map<String, Value> {
    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                return Map::new();
            }
            _ => panic!("Unable to open the file: {:?}", error),
        },
    };

    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Unable to read the file");

    if data.is_empty() {
        return Map::new();
    }

    let json: Value = serde_json::from_str(&data).expect("Unable to parse JSON");
    let state: Map<String, Value> = json.as_object().expect("Expected JSON object").clone();

    state
}

pub fn write_to_file(file_name: &str, state: &mut Map<String, Value>) {
    let new_data = json!(state);
    fs::write(file_name, new_data.to_string()).expect("Unable to write to file");
}
