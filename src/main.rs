use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::{env, io};

use gcviz::app::{App, AppResult, LogDestination};
use gcviz::event::{Event, EventHandler};
use gcviz::file_utils::load_program_from_file;
use gcviz::frame::Program;
use gcviz::gc::GCType;
use gcviz::handler::handle_key_events;
use gcviz::simulator::{Parameters, Simulator};
use gcviz::tui::Tui;

static TICK_RATE: u64 = 100;
static NUM_FRAMES: usize = 100; // program size
static ALIGNMENT: usize = 4;
static HEAP_SIZE: usize = 1024;

fn main() -> AppResult<()> {
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

    let mut app = App::new(
        HEAP_SIZE,
        ALIGNMENT,
        &gc_type,
        program,
        sim_params,
        LogDestination::EventStream,
    );

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(TICK_RATE);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => {
                if let Err(e) = app.tick() {
                    panic!("main: {e:?}");
                }
            }
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
