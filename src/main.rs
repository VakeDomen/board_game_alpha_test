use std::{
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread::spawn, collections::HashMap,
};

use once_cell::sync::Lazy;
use tokio_tungstenite::tungstenite::accept;

mod game;

static UNKNOWN_SOCKETS: Lazy<Mutex<Vec<TcpStream>>> = Lazy::new(|| Mutex::new(vec![]));
static NAMED_SOCKETS: Lazy<Mutex<HashMap<String, TcpStream>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[tokio::main]
async fn main() {
    let listen_addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(&listen_addr).unwrap();
    println!("Listening on: {}", listen_addr);
    

    for stream_result in listener.incoming() {
        if let Ok(stream) = stream_result {
            handle_new_connection(stream);
        }
        println!("New connection");
    };
    println!("Server ended");
}

fn handle_new_connection(stream: TcpStream) {
    let _handle = spawn(move || {
        let mut websocket = accept(stream).unwrap();
        
        // Add the connection to the list
        {
            let mut conns = UNKNOWN_SOCKETS.lock().unwrap();
            conns.push(websocket.get_ref().try_clone().unwrap());
        }
        
        loop {
            let msg = match websocket.read() {
                Ok(msg) => msg,
                Err(e) => {
                    println!("Socket read error: {:#?}", e);
                    break;
                }
            };

            if msg.is_close() {
                println!("Closing socket!");
                break;
            }

            // Echo the message back
            if msg.is_binary() || msg.is_text() {
                println!("MSG is data");
                if let Err(e) = websocket.write(msg) {
                    println!("Something went wrong sending msg to WS clinet: {:#?}", e)
                };

                let _ = websocket.flush();
            } 

        }

        // Cleanup: remove the closed connection from the list
        let mut conns = UNKNOWN_SOCKETS.lock().unwrap();
        conns.retain(|conn| !conn.peer_addr().is_err());
        
        // Cleanup: remove the closed connection from the list
        let mut named_sockets = NAMED_SOCKETS.lock().unwrap();
        // Retain only the sockets that are still open and haven't encountered errors
        named_sockets.retain(|_name, socket| {
            // Check the state of the socket, for example, by checking if it's readable
            // This is a placeholder for your logic to determine if the socket should be retained
            socket.peer_addr().is_ok()
        });
    });
}
