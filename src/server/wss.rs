use std::{net::{TcpListener, TcpStream}, thread::spawn};
use tokio_tungstenite::tungstenite::{accept, Message, Error};
use uuid::Uuid;

use crate::storage::active::SOCKETS;
use super::{message::WSSMessage, message_handler::handle};


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
        let websocket = accept(stream).unwrap();
        let socket_id = Uuid::new_v4().to_string();
    
        // Add the connection to the list
        {
            let mut conns = SOCKETS.lock().unwrap();
            conns.insert(socket_id.clone(), (websocket, None));
        }
        
        loop {
            let raw_msg: Option<Message> = match get_message(&socket_id) {
                Ok(m) => m,
                Err(_) => {
                    remove_socket(&socket_id);
                    break;
                }
            };

            if raw_msg.is_none() {
                continue;
            }

            let raw_msg = raw_msg.unwrap();
 
            if raw_msg.is_close() {
                println!("Closing socket!");
                remove_socket(&socket_id);
                break;
            }

            if raw_msg.is_binary() || raw_msg.is_text() {
                let msg = WSSMessage::from(raw_msg);
                let reponse = handle(msg, socket_id.clone());
                if let Err(_) = send_message(&socket_id,reponse) {
                    remove_socket(&socket_id);
                    break;
                }
            } 
        }
    });
}


fn get_message(socket_id: &String) -> Result<Option<Message>, Error> {
    let mut msg = None;
    let mut sockets = SOCKETS.lock().unwrap();
    let ws_option = sockets.get_mut(socket_id);
    if let Some(ws) = ws_option {
        match ws.0.read() {
            Ok(raw_msg) => msg = Some(raw_msg),
            Err(e) => {
                println!("Socket read error: {:#?}", e);
                return Err(e.into());
            }
        }
    }
    Ok(msg)
}

fn send_message(socket_id: &String, msg: WSSMessage) -> Result<(), Error> {
    let mut sockets = SOCKETS.lock().unwrap();
    let ws_option = sockets.get_mut(socket_id);
    if let Some(ws) = ws_option {
        if let Err(e) = ws.0.write(msg.into()) {
            println!("Something went wrong sending raw_msg to WS clinet: {:#?}", e);
            return Err(e.into())
        };
    
        let _ = ws.0.flush();
    }
    Ok(())
}

fn remove_socket(socket_id: &String) {
    let mut sockets = SOCKETS.lock().unwrap();
    sockets.remove(socket_id);
}