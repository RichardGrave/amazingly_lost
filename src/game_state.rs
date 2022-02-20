use bevy::{asset::HandleId, prelude::*};
use bevy::ecs::prelude::Res;

pub struct ChangeGameStateEvent(pub GameState);

pub struct ChangeGameStatePlugin;

impl Plugin for ChangeGameStatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(check_game_state_change.system());
    }
}

pub fn check_game_state_change(
    mut read_game_state: EventReader<ChangeGameStateEvent>,
    mut game_state: ResMut<State<GameState>>,
) {
    for ev in read_game_state.iter() {
        if *game_state.current() != ev.0 {
            game_state.set(ev.0).unwrap();
        }
    }
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    StartMenu,
    // Story,
    PlayingGame,
    // Pause,
    // Win,
    Settings,
    Save,
    // About,
    GenerateNewGame,
    // Loading,
    LoadingAssets
}