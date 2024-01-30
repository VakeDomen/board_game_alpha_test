use crate::{
    storage::operations::{game::get_other_player, socket::authenticate_socket}, 
    server::messages::{wss_message::WSSMessage, control_commands::ControlCommand, game_commands::GameCommand}
};

use super::{control_message::{create_game, join_game, start_game, list_lobby, list_running}, game_message::{get_state, setup_base, next_phase, undo_move, get_recepies, activate_ability, apply_phase, place_tile, damage}};


pub fn handle(msg: WSSMessage, socket_id: String) -> WSSMessage {
    match msg {
        WSSMessage::Game(name, g) => {
            let msg = handle_game_message(name.clone(), g);
            let other_player = get_other_player(name, socket_id);
            return msg;
        },
        WSSMessage::Control(c) => handle_control_message(c, socket_id),
        _ => return WSSMessage::Unknown,
    }
}

fn handle_control_message(msg: ControlCommand, socket_id: String) -> WSSMessage {
    match msg {
        ControlCommand::Authenticate(name) => authenticate_socket(name, socket_id),
        ControlCommand::CreateGame(name) => create_game(name, socket_id),
        ControlCommand::JoinGame(name) => join_game(name, socket_id),
        ControlCommand::StartGame(name) => start_game(name, socket_id),
        ControlCommand::ListLobby(name) => list_lobby(name, socket_id),
        ControlCommand::ListRunning(name) => list_running(name, socket_id),
        ControlCommand::Unknown => WSSMessage::Unknown,
    }
}

fn handle_game_message(game_name: String, msg: GameCommand) -> WSSMessage {
    match msg {
        GameCommand::GetState => get_state(game_name),
        GameCommand::BaseSetup(x, y) => setup_base(game_name, x, y),
        GameCommand::Dmg(initiator, target, dmg) => damage(game_name, initiator, target, dmg),
        GameCommand::PlaceTile(selector, x, y, rotate) => place_tile(game_name, selector, x, y, rotate),
        GameCommand::NextPhase => next_phase(game_name),
        GameCommand::ApplyPhase => apply_phase(game_name),
        GameCommand::GetRecepies => get_recepies(game_name),
        GameCommand::Undo => undo_move(game_name),
        GameCommand::InvalidCommand(e) => WSSMessage::Error(e),
        GameCommand::ActivateAbility(tile_id, ability_index, additional_data) => activate_ability(game_name, tile_id, ability_index, additional_data),
    }
}
