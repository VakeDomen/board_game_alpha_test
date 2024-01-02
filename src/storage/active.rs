use std::{sync::Mutex, net::TcpStream, collections::HashMap};
use once_cell::sync::Lazy;
use tokio_tungstenite::tungstenite::WebSocket;
use serde_any::Format;
use std::{fs, io};

use crate::game::core::lobby::match_state::MatchState;

pub type SocketData<'a> = (WebSocket<TcpStream>, Option<String>);

pub static SOCKETS: Lazy<Mutex<HashMap<String, SocketData>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static MATCHES: Lazy<Mutex<Vec<MatchState>>> = Lazy::new(|| {
    match serde_any::from_file("matches.json") {
        Ok(hm) => Mutex::new(hm),
        Err(_) => Mutex::new(vec![])
    }
});

pub fn save_matches() {
    let matches = MATCHES.lock().unwrap();
    match serde_any::to_file("matches.json", &*matches) {
        Ok(_) => { println!("Matches saved.") },
        Err(e) => {println!("Error saving matches: {:?}", e);}
    };
}

pub fn save_matches_in_scope<T: serde::Serialize>(data: &T) -> io::Result<()> {
    let serialized = serde_any::to_string(data, Format::Json).unwrap();
    fs::write("matches.json", serialized)
}
