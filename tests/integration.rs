use std::env;

#[cfg(test)]
use pretty_assertions::assert_eq;
use serde_json::to_value;

use gcviz::{
    error::VMError,
    file_utils::{load_heap_snapshot, load_program, save_heap_snapshot, CURRENT_DIR},
    session::Session,
};

fn init_test(test_name: &str) -> Session {
    let (program, rts_cfg) = load_program(test_name);
    let mut session = Session::new(rts_cfg);
    session.program = program;
    session
}

fn run_test(test: &mut Session) -> Result<(), VMError> {
    while test.program.get(test.instr_ptr).is_some() {
        match test.tick() {
            Err(e) => {
                return Err(e);
            }
            _ => {}
        }
    }
    Ok(())
}

fn check_against_snapshot(test_app: &Session, test_name: &str) {
    let result_value = to_value(&test_app.vm.heap).unwrap();
    let heap_snapshot = load_heap_snapshot(test_name);
    let snapshot_value = to_value(&heap_snapshot).unwrap();
    assert_eq!(snapshot_value, result_value);
}

fn __test(test_name: &str) -> Result<(), VMError> {
    let update_snapshots = env::var("UPDATE_SNAPSHOTS").is_ok();
    let mut test_app = init_test(test_name);
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
    assert!(__test("simple").is_ok());
}

#[test]
fn test_could_not_allocate() {
    assert!(__test("could_not_allocate").is_err());
}

#[test]
fn test_alignment_alloc_fail() {
    assert!(__test("alignment_alloc_fail").is_err());
}

#[test]
fn test_alloc_1() {
    assert!(__test("alloc_1").is_ok());
}
