use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ControlCommand {
    Authenticate(String),
    CreateGame(String),
    JoinGame(String),
    StartGame(String),
    ListLobby(String),
    ListRunning(String),
    Unknown,
}

impl From<String> for ControlCommand {
    fn from(value: String) -> Self {
        match value.as_str() {
            _ if value.starts_with("AUTH ") => Self::Authenticate(value["AUTH ".len()..].to_string()),
            _ if value.starts_with("CREATE ") => Self::CreateGame(value["CREATE ".len()..].to_string()),
            _ if value.starts_with("JOIN ") => Self::JoinGame(value["JOIN ".len()..].to_string()),
            _ if value.starts_with("START ") => Self::StartGame(value["START ".len()..].to_string()),
            _ if value.starts_with("LOBBY") => Self::ListLobby(value["START ".len()..].to_string()),
            _ if value.starts_with("RUNNING") => Self::ListRunning(value["START ".len()..].to_string()),
            _ => Self::Unknown,
        }
    }
}