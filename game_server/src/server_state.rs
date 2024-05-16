use crate::in_game::in_game_data::InGameData;

#[derive(Default)]
pub enum ServerState {
    #[default]
    Lobby,
    InGame(InGameData),
}
