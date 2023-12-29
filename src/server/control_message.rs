use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ControlMessage {
    Authenticate(String),
    CreateGame(String),
    JoinGame(String),
    StartGame(String),
    Unknown,
}

impl From<String> for ControlMessage {
    fn from(value: String) -> Self {
        match value.as_str() {
            _ if value.starts_with("AUTH ") => Self::Authenticate(value["AUTH ".len()..].to_string()),
            _ if value.starts_with("CREATE ") => Self::CreateGame(value["CREATE ".len()..].to_string()),
            _ if value.starts_with("JOIN ") => Self::JoinGame(value["JOIN ".len()..].to_string()),
            _ if value.starts_with("START ") => Self::StartGame(value["START ".len()..].to_string()),
            _ => Self::Unknown,
        }
    }
}