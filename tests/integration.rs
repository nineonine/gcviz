use std::{env, path::PathBuf};

use lazy_static::lazy_static;
#[cfg(test)]
use pretty_assertions::assert_eq;
use serde_json::to_value;

use gcviz::{
    app::{App, LogDestination},
    error::VMError,
    file_utils::{load_heap_from_file, load_program_from_file, save_heap_snapshot},
    frame::Program,
    gc::GCType,
    heap::Heap,
    simulator::Parameters,
};

lazy_static! {
    pub static ref CURRENT_DIR: PathBuf = env::current_dir().unwrap();
}

fn load_program(file_name: &str) -> Program {
    let path = format!("{}/tests/{file_name}.yaml", CURRENT_DIR.display());
    load_program_from_file(path.as_str()).unwrap()
}

fn load_heap_snapshot(file_name: &str) -> Heap {
    let path = format!("{}/tests/{file_name}_snapshot.yaml", CURRENT_DIR.display());
    load_heap_from_file(path.as_str()).unwrap()
}

fn init_test(test_name: &str, heap_size: usize, alignment: usize, gc_type: GCType) -> App {
    let program: Program = load_program(test_name);
    let sim_params: Parameters = Parameters::new(heap_size, alignment, program.len());
    App::new(
        heap_size,
        alignment,
        &gc_type,
        program,
        sim_params,
        LogDestination::Stdout,
    )
}

fn run_test(test: &mut App) -> Result<(), VMError> {
    while test.program.get(test.frame_ptr).is_some() {
        match test.tick() {
            Err(e) => {
                return Err(e);
            }
            _ => {}
        }
    }
    Ok(())
}

fn check_against_snapshot(test_app: &App, test_name: &str) {
    let result_value = to_value(&test_app.vm.heap).unwrap();
    let heap_snapshot = load_heap_snapshot(test_name);
    let snapshot_value = to_value(&heap_snapshot).unwrap();
    assert_eq!(snapshot_value, result_value);
}

fn __test(
    test_name: &str,
    heap_size: usize,
    alignment: usize,
    gc_type: GCType,
) -> Result<(), VMError> {
    let update_snapshots = env::var("UPDATE_SNAPSHOTS").is_ok();

    let mut test_app = init_test(test_name, heap_size, alignment, gc_type);
    run_test(&mut test_app)?;
    if update_snapshots {
        // save snapshot and don't compare
        let path = format!("{}/tests/{test_name}", CURRENT_DIR.display());
        save_heap_snapshot(&test_app.vm.heap, &path).unwrap();
    } else {
        check_against_snapshot(&test_app, test_name);
    }
    Ok(())
}

#[test]
fn test_simple() {
    assert!(__test("simple", 4, 0, GCType::MarkSweep).is_ok());
}

#[test]
fn test_could_not_allocate() {
    assert!(__test("could_not_allocate", 1, 0, GCType::MarkSweep).is_err());
}
