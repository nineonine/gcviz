use futures_util::{SinkExt, StreamExt};
use log::*;
use std::env;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{
    accept_async,
    tungstenite::{Error, Result},
};
use tungstenite::Message;

use gcviz::file_utils::load_program_from_file;
use gcviz::frame::Program;
use gcviz::gc::GCType;
use gcviz::session::{LogDestination, Session, SessionResult};
use gcviz::simulator::{Parameters, Simulator};

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

    while let Some(msg) = ws_stream.next().await {
        let msg = msg?;
        if msg.is_text() || msg.is_binary() {
            if let Err(e) = session.tick() {
                error!("tick panic: {}", e);
            }

            // Serialize the heap's memory and send it to the client.
            let serialized_memory = serde_json::to_string(&session.vm.heap.memory)
                .expect("Failed to serialize heap memory");
            ws_stream.send(Message::Text(serialized_memory)).await?;
            ws_stream.send(msg).await?;
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
