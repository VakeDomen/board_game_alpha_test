use crate::{server::messages::{game_commands::GameCommand, wss_message::WSSMessage}, storage::operations::game::get_running_game_by_name};


pub fn get_state(game_name: String, msg: GameCommand) -> WSSMessage {
    let game = get_running_game_by_name(&game_name);
    WSSMessage::State(game.unwrap().clone())
}
