use std::{sync::Mutex, net::TcpStream, collections::HashMap};

use once_cell::sync::Lazy;
use tokio_tungstenite::tungstenite::WebSocket;

use crate::game::match_state::MatchState;

pub type SocketData<'a> = (WebSocket<TcpStream>, Option<String>);

pub static SOCKETS: Lazy<Mutex<HashMap<String, SocketData>>> = Lazy::new(|| Mutex::new(HashMap::new()));
pub static MATCHES: Lazy<Mutex<Vec<MatchState>>> = Lazy::new(|| Mutex::new(vec![]));
