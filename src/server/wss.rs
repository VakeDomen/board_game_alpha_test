use std::{net::{TcpListener, TcpStream}, thread::spawn};

use tokio_tungstenite::tungstenite::accept;

use crate::storage::active::{UNKNOWN_SOCKETS, NAMED_SOCKETS};

use super::message::WSSMessage;


pub fn start_server(listen_addr: &str) {
    let listener = TcpListener::bind(&listen_addr).unwrap();
    println!("WSS listening on: {}", listen_addr);

    for stream_result in listener.incoming() {
        if let Ok(stream) = stream_result {
            handle_new_connection(stream);
        }
        println!("New WSS connection");
    };
    println!("Server ended");
}


pub fn handle_new_connection(stream: TcpStream) {
    let _handle = spawn(move || {
        let mut websocket = accept(stream).unwrap();
        
        // Add the connection to the list
        {
            let mut conns = UNKNOWN_SOCKETS.lock().unwrap();
            conns.push(websocket.get_ref().try_clone().unwrap());
        }
        
        loop {
            let raw_msg = match websocket.read() {
                Ok(raw_msg) => raw_msg,
                Err(e) => {
                    println!("Socket read error: {:#?}", e);
                    break;
                }
            };

            if raw_msg.is_close() {
                println!("Closing socket!");
                break;
            }

            // Echo the message back
            if raw_msg.is_binary() || raw_msg.is_text() {
                
                let msg = WSSMessage::from(raw_msg);
                
                if let Err(e) = websocket.write(msg.into()) {
                    println!("Something went wrong sending raw_msg to WS clinet: {:#?}", e)
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
