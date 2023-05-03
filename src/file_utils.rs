use serde_json::Error as SerdeJsonError;
use serde_yaml::Error as SerdeYamlError;
use std::error::Error;
use std::fmt;

use std::{
    fs::File,
    io::{Read, Write},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{frame::Program, heap::Heap};

#[derive(Debug)]
pub enum CustomError {
    Json(SerdeJsonError),
    Yaml(SerdeYamlError),
}

impl Error for CustomError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CustomError::Json(e) => Some(e),
            CustomError::Yaml(e) => Some(e),
        }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomError::Json(e) => write!(f, "JSON error: {e}"),
            CustomError::Yaml(e) => write!(f, "YAML error: {e}"),
        }
    }
}

impl From<SerdeJsonError> for CustomError {
    fn from(error: SerdeJsonError) -> Self {
        CustomError::Json(error)
    }
}

impl From<SerdeYamlError> for CustomError {
    fn from(error: SerdeYamlError) -> Self {
        CustomError::Yaml(error)
    }
}

pub fn save_program_to_file(program: &Program) -> Result<String, CustomError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let filename = format!("program_{now}.yaml");
    let yaml_program = serde_yaml::to_string(&program)?;
    let mut file = File::create(&filename).expect("Failed to create file");
    file.write_all(yaml_program.as_bytes())
        .expect("Failed to write to file");

    Ok(filename)
}

pub fn save_heap_snapshot(heap: &Heap, test_path: &str) -> Result<String, CustomError> {
    let filename = format!("{test_path}_snapshot.yaml");
    let yaml_heap = serde_yaml::to_string(&heap)?;
    let mut file = File::create(&filename).expect("Failed to create file");
    file.write_all(yaml_heap.as_bytes())
        .expect("Failed to write to file");

    Ok(filename)
}

pub fn load_program_from_file(filename: &str) -> Result<Program, CustomError> {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read program from file");
    serde_yaml::from_str(&contents).map_err(CustomError::from)
}

pub fn load_heap_from_file(filename: &str) -> Result<Heap, CustomError> {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read heap from file");
    serde_yaml::from_str(&contents).map_err(CustomError::from)
}
