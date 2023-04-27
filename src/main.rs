use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use serde_json::Error as SerdeError;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

use gcviz::app::{App, AppResult};
use gcviz::event::{Event, EventHandler};
use gcviz::frame::Program;
use gcviz::gc::GCType;
use gcviz::handler::handle_key_events;
use gcviz::simulator::{Parameters, Simulator};
use gcviz::tui::Tui;

static TICK_RATE: u64 = 100;
static NUM_FRAMES: usize = 100;

fn main() -> AppResult<()> {
    // Check command line arguments for a program file name.
    let args: Vec<String> = env::args().collect();
    // Create an application.
    let gc_type = GCType::MarkSweep;
    let program: Program = if args.len() > 1 {
        load_program_from_file(&args[1])?
    } else {
        let mut sim = Simulator::new(Parameters::new(NUM_FRAMES), &gc_type);
        let program = sim.gen_program();
        // match save_program_to_file(&program) {
        //     Ok(filename) => println!("Program saved to {}", filename),
        //     Err(e) => eprintln!("Failed to save program: {}", e),
        // }
        program
    };

    let mut app = App::new(4, 1024, &gc_type, program);

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
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}

fn save_program_to_file(program: &Program) -> Result<String, SerdeError> {
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

fn load_program_from_file(filename: &str) -> Result<Program, SerdeError> {
    let mut file = File::open(filename).expect("Failed to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file");
    serde_json::from_str(&contents)
}
