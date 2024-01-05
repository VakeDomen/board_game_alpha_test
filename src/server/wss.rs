use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{tungstenite::{Result, Message, Error}, accept_async};
use uuid::Uuid;

use crate::storage::{active::SOCKETS, operations::socket::{remove_socket, send_message, get_message}};
use super::{messages::wss_message::WSSMessage, handlers::wss_message::handle};


pub async fn start_server(listen_addr: &str) {
    let listener = TcpListener::bind(&listen_addr).await.expect("Can't listen");

    while let Ok((stream, _)) = listener.accept().await {
        println!("New Connection");
        let peer = stream.peer_addr().expect("connected streams should have a peer address");
        tokio::spawn(accept_connection(peer, stream));
    }
    println!("Server ended");
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => println!("Error processing connection: {}", err),
        }
    }
}

pub async fn handle_connection(_: SocketAddr, stream: TcpStream) -> Result<()>  {
    let mut websocket = accept_async(stream).await.expect("Failed to accept");
    let socket_id = Uuid::new_v4().to_string();
    // Add the connection to the list
    {
        let mut conns = SOCKETS.lock().unwrap();
        conns.insert(socket_id.clone(), None);
    }
    
    loop {
        let raw_msg: Option<Message> = match get_message(&mut websocket).await {
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
            if let Err(_) = send_message(&mut websocket,reponse).await {
                remove_socket(&socket_id);
                break;
            }
        } 
    }
    Ok(())
}

