use futures_util::{SinkExt, StreamExt};
use log::{debug, error, info};
use std::env;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Result},
};
use tungstenite::Message;

use gcviz::gc::GCType;
use gcviz::session::{LogDestination, Session, SessionResult};
use gcviz::simulator::{Parameters, Simulator};
use gcviz::{file_utils::load_program_from_file, wsmsg::WSMessageResponse};
use gcviz::{
    frame::Program,
    wsmsg::{WSMessageRequest, WSMessageRequestType},
};

static NUM_FRAMES: usize = 100;
static ALIGNMENT: usize = 4;
static HEAP_SIZE: usize = 1024;

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let mut ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let mut session: Session = init_session().unwrap();
    let mut already_said_halt: bool = false;

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            let request: Result<WSMessageRequest, _> = serde_json::from_str(msg.to_text()?);
            match request {
                Ok(msg) => match msg.msg_type {
                    WSMessageRequestType::TICK => {
                        if let Err(e) = session.tick() {
                            error!("tick panic: {}", e);
                        }
                        let last_log_entry = session.logs.back().cloned().clone();
                        debug!("{} {:?}", session.instr_ptr, last_log_entry);
                        // Serialize the heap's memory and send it to the client.
                        let msg_resp = WSMessageResponse::new_tick(
                            session.vm.heap.memory.clone(),
                            last_log_entry,
                            msg.pause_on_return,
                        );
                        let serialized_memory = serde_json::to_string(&msg_resp)
                            .expect("Failed to serialize Tick message");
                        ws_stream.send(Message::Text(serialized_memory)).await?;

                        // All instructions/events processed - stop program execution
                        if session.instr_ptr == session.program.len() && !already_said_halt {
                            debug!("Program halted");
                            let halt_msg = serde_json::to_string(&WSMessageResponse::halt())
                                .expect("Failed to serialize Halt message");
                            ws_stream.send(Message::Text(halt_msg)).await?;
                            already_said_halt = true;
                        }
                    }
                    WSMessageRequestType::RESET => {
                        already_said_halt = false;
                        session.restart();
                    }
                },
                Err(e) => {
                    error!("Failed to deserialize message: {}", e);
                }
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> SessionResult<()> {
    env_logger::init();
    info!("RUN");

    // WebSocket server setup
    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");
        info!("Peer address: {}", peer);
        tokio::spawn(accept_connection(peer, stream));
    }

    Ok(())
}

fn init_session() -> SessionResult<Session> {
    // Check command line arguments for a program file name.
    let args: Vec<String> = env::args().collect();
    // Create an running program session.
    let sim_params = Parameters::new(HEAP_SIZE, ALIGNMENT, NUM_FRAMES);
    let gc_type = GCType::MarkSweep;
    let program: Program = if args.len() > 1 {
        load_program_from_file(&args[1])?
    } else {
        let mut sim = Simulator::new(sim_params.clone(), &gc_type);
        info!("Generating program using simulation params");
        sim.gen_program()
        // match save_program_to_file(&program) {
        //     Ok(filename) => println!("Program saved to {}", filename),
        //     Err(e) => eprintln!("Failed to save program: {}", e),
        // }
    };

    let session = Session::new(
        HEAP_SIZE,
        ALIGNMENT,
        &gc_type,
        program,
        sim_params,
        LogDestination::EventStream,
    );

    Ok(session)
}
