use gcviz::app::{App, AppResult};
use gcviz::event::{Event, EventHandler};
use gcviz::gc::collector::GCType;
use gcviz::handler::handle_key_events;
use gcviz::simulator::{Parameters, Simulator};
use gcviz::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;

fn main() -> AppResult<()> {
    // Create an application.
    let gc_type = GCType::MarkSweep;
    let mut sim = Simulator::new(Parameters::default(), &gc_type);
    let program = sim.gen_program();
    let mut app = App::new(0, 1024, &gc_type, program);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(100);
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
