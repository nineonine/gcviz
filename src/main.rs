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

use gcviz::{file_utils, wsmsg::WSMessageResponse};
use gcviz::{file_utils::CustomError, gc::init_collector};
use gcviz::{
    program::Program,
    wsmsg::{WSMessageRequest, WSMessageRequestType},
};
use gcviz::{
    rts_cfg::ProgramRuntimeConfig,
    session::{Session, SessionResult},
};

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
    let mut already_said_halt: bool = false;
    let mut session: Session = Session::default();

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            let request: Result<WSMessageRequest, _> = serde_json::from_str(msg.to_text()?);
            match request {
                Ok(msg) => match msg.msg_type {
                    WSMessageRequestType::LoadProgram => {
                        load_program(&mut session, msg.program_name).unwrap();
                    }
                    WSMessageRequestType::Tick => {
                        if already_said_halt {
                            // if program execution is done - don't execute
                            continue;
                        }
                        // All instructions/events processed - stop program execution
                        if session.instr_ptr == session.program.len() && !already_said_halt {
                            info!("Program halted");
                            let halt_msg = serde_json::to_string(&WSMessageResponse::halt())
                                .expect("Failed to serialize Halt message");
                            ws_stream.send(Message::Text(halt_msg)).await?;
                            already_said_halt = true;
                            continue;
                        }
                        match session.tick() {
                            Ok(instr_result) => {
                                let last_log_entry = session.logs.back().cloned();
                                debug!(
                                    "ip:{} [TICK]: {:?}; Result: {:?}",
                                    session.instr_ptr, last_log_entry, instr_result
                                );
                                // Serialize the heap's memory and send it to the client.
                                let msg_resp = WSMessageResponse::new_tick(
                                    session.vm.heap.memory.clone(),
                                    last_log_entry,
                                    msg.pause_on_return,
                                    Some(instr_result),
                                    session.make_info_block(),
                                );
                                let serialized_memory = serde_json::to_string(&msg_resp)
                                    .expect("Failed to serialize Tick message");
                                ws_stream.send(Message::Text(serialized_memory)).await?;
                            }
                            Err(e) => {
                                error!("tick panic: {}", e);
                            }
                        }
                    }
                    WSMessageRequestType::Reset => {
                        info!("Program reset");
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

///
/// 1. If `file_name` is provided, it loads the program from the specified file.
/// 2. If `file_name` is not provided, it checks the environment variable `PROGRAM_FILE`
///    for a file name and attempts to load the program from this file.
/// 3. If neither `file_name` nor the environment variable provide a valid source,
///    the function generates a random program.
///
fn load_program(session: &mut Session, file_name: Option<String>) -> Result<(), CustomError> {
    let (program, rts_cfg): (Program, ProgramRuntimeConfig) = if let Some(fname) = file_name {
        // Load program using provided file name.
        info!("Loading program from provided file name: {}", fname);
        file_utils::load_program(&fname)
    } else if let Ok(env_file) = env::var("PROGRAM_FILE") {
        // Load program from environment variable.
        info!("Loading program from environment variable: {}", env_file);
        file_utils::load_program(&env_file)
    } else {
        // Generate a new program.
        info!("Generating program using simulation params");
        session.gen_program()
    };
    session.program = program;
    session.vm.reset_heap(rts_cfg.heap_size);
    session.rts_cfg = rts_cfg;
    session.vm.allocator.alignment = session.rts_cfg.alignment;
    session.vm.collector = init_collector(&session.rts_cfg.gc_ty);
    assert!(session.rts_cfg.alignment == session.vm.allocator.alignment);
    Ok(())
}
