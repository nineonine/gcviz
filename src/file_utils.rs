use serde_json::Error as SerdeError;
use std::{
    fs::File,
    io::{Read, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{frame::Program, heap::Heap};

pub fn save_program_to_file(program: &Program) -> Result<String, SerdeError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let filename = format!("program_{now}.json");
    let json_program = serde_json::to_string_pretty(program)?;
    let mut file = File::create(&filename).expect("Failed to create file");
    file.write_all(json_program.as_bytes())
        .expect("Failed to write to file");

    Ok(filename)
}

pub fn save_heap_snapshot(heap: &Heap, test_path: &str) -> Result<String, SerdeError> {
    let filename = format!("{test_path}_snapshot.json");
    let json_heap = serde_json::to_string_pretty(heap)?;
    let mut file = File::create(&filename).expect("Failed to create file");
    file.write_all(json_heap.as_bytes())
        .expect("Failed to write to file");

    Ok(filename)
}

pub fn load_program_from_file(filename: &str) -> Result<Program, SerdeError> {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read program from file");
    serde_json::from_str(&contents)
}

pub fn load_heap_from_file(filename: &str) -> Result<Heap, SerdeError> {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read heap from file");
    serde_json::from_str(&contents)
}
