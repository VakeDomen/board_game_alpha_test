use std::collections::HashMap;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use crate::game::core::lobby::match_state::MatchState;

pub static SOCKETS: Lazy<Mutex<HashMap<String, Option<String>>>> = Lazy::new(|| Mutex::new(HashMap::new()));
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