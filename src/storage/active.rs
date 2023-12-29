use std::{sync::Mutex, net::TcpStream, collections::HashMap};

use once_cell::sync::Lazy;

pub static UNKNOWN_SOCKETS: Lazy<Mutex<Vec<TcpStream>>> = Lazy::new(|| Mutex::new(vec![]));
pub static NAMED_SOCKETS: Lazy<Mutex<HashMap<String, TcpStream>>> = Lazy::new(|| Mutex::new(HashMap::new()));
