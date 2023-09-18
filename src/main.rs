use std::env;

use gcviz::session::{Session, SessionResult, LogDestination};
use gcviz::file_utils::load_program_from_file;
use gcviz::frame::Program;
use gcviz::gc::GCType;
use gcviz::simulator::{Parameters, Simulator};

static NUM_FRAMES: usize = 100; // program size
static ALIGNMENT: usize = 4;
static HEAP_SIZE: usize = 1024;

fn main() -> SessionResult<()> {
    // Check command line arguments for a program file name.
    let args: Vec<String> = env::args().collect();
    // Create an application.
    let sim_params = Parameters::new(HEAP_SIZE, ALIGNMENT, NUM_FRAMES);
    let gc_type = GCType::MarkSweep;
    let program: Program = if args.len() > 1 {
        load_program_from_file(&args[1])?
    } else {
        let mut sim = Simulator::new(sim_params.clone(), &gc_type);
        sim.gen_program()
        // match save_program_to_file(&program) {
        //     Ok(filename) => println!("Program saved to {}", filename),
        //     Err(e) => eprintln!("Failed to save program: {}", e),
        // }
    };

    let mut session = Session::new(
        HEAP_SIZE,
        ALIGNMENT,
        &gc_type,
        program,
        sim_params,
        LogDestination::EventStream,
    );

    // Start the main loop.
    while true {
        if let Err(e) = session.tick() {
            panic!("main: {e:?}");
        }
    }

    // Exit the user interface.
    Ok(())
}
