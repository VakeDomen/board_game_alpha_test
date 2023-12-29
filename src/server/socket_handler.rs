use tokio_tungstenite::tungstenite::{Error, Message};

    use crate::{server::message::WSSMessage, storage::active::SOCKETS};

    pub fn get_socket_name(socket_id: &String) -> Option<String> {
        let socket_data = SOCKETS.lock().unwrap();
        let s = socket_data.get(socket_id);
        if let Some(s) = s {
            return s.1.clone();
        }
        None
    }

    pub fn authenticate_socket(new_name: String, socket_id: String) -> WSSMessage {
        let mut socket_data = SOCKETS.lock().unwrap();
        for (id, data) in socket_data.iter_mut() {
            if *id == socket_id {
                data.1 = Some(new_name.clone()); // Dereference to modify
                return WSSMessage::Success(true)
            }
        }
        WSSMessage::Success(false) // Assuming this is a valid return value
    }
    
    pub fn is_authenticated(socket_id: &String) -> bool {
        let socket_data = SOCKETS.lock().unwrap();
        let s = socket_data.get(socket_id);
        if let Some(s) = s {
            return s.1.is_some();
        }
        false
    }

    pub fn get_message(socket_id: &String) -> Result<Option<Message>, Error> {
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
    
    pub fn send_message(socket_id: &String, msg: WSSMessage) -> Result<(), Error> {
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
    
    pub fn remove_socket(socket_id: &String) {
        let mut sockets = SOCKETS.lock().unwrap();
        sockets.remove(socket_id);
    }