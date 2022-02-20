use crate::maze_generator;
use crate::{amazingly_lost_data, player};
use crate::{amazingly_lost_data::AmazinglyLostData, player::Player};

use crate::game_state::{ChangeGameStateEvent, GameState};
use crate::maze_generator::{
    CollisionTile, GameTile, PlayerTile, SMALL_MAZE, VERY_VERY_LARGE_MAZE,
};
use crate::maze_tile::MazeTile;
use crate::player::ChangeDirectionEvent;
use crate::tile_factory::GameTileHandlers;
use bevy::app::AppExit;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::camera::{Camera, OrthographicProjection};
use player::Directions;

pub struct KeyboardInputPlugin;

impl Plugin for KeyboardInputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // We need a small input delay or the player will move to fast
        app.add_system(keyboard_input_game.system());
    }
}

pub fn keyboard_input_game(
    keyboard_input: Res<Input<KeyCode>>,
    // mut camera_query: Query<(&mut OrthographicProjection, (With<Camera>, (Without<CollisionTile>, Without<PlayerTile>)))>,
    mut player_query: Query<(
        &mut Player,
        (With<PlayerTile>, (Without<CollisionTile>, Without<Camera>)),
    )>,
    mut amazing_data: ResMut<AmazinglyLostData>,
    mut game_state: Res<State<GameState>>,
    mut change_game_state: EventWriter<ChangeGameStateEvent>,
    mut change_direction: EventWriter<ChangeDirectionEvent>,
    mut exit: EventWriter<AppExit>,
) {
    // Only when playing a game and the player is NOT already moving
    if *game_state.current() == GameState::PlayingGame {
        check_for_player_movement(&keyboard_input, &mut player_query, &mut change_direction);

        if keyboard_input.just_pressed(KeyCode::N) {
            println!("New Game");
            change_game_state.send(ChangeGameStateEvent(GameState::GenerateNewGame));
        } else if keyboard_input.just_pressed(KeyCode::Q)
            || keyboard_input.just_pressed(KeyCode::Escape)
        {
            println!("Exit Game");
            exit.send(AppExit);
        } else if keyboard_input.just_pressed(KeyCode::PageUp) {
            if amazing_data.maze_size.0 < VERY_VERY_LARGE_MAZE {
                println!("Bigger maze");
                amazing_data.maze_size =
                    (amazing_data.maze_size.0 + 33, amazing_data.maze_size.1 + 33);
                println!(
                    "SIZE: {:?}{:?}",
                    amazing_data.maze_size,
                    amazing_data.maze_size.0 as usize * amazing_data.maze_size.1 as usize
                );
                change_game_state.send(ChangeGameStateEvent(GameState::GenerateNewGame));
            }
        } else if keyboard_input.just_pressed(KeyCode::PageDown) {
            if amazing_data.maze_size.0 > SMALL_MAZE {
                println!("Smaller maze");
                amazing_data.maze_size =
                    (amazing_data.maze_size.0 - 33, amazing_data.maze_size.1 - 33);
                println!(
                    "SIZE: {:?}{:?}",
                    amazing_data.maze_size,
                    amazing_data.maze_size.0 as usize * amazing_data.maze_size.1 as usize
                );
                change_game_state.send(ChangeGameStateEvent(GameState::GenerateNewGame));
            }
        }
    }
}

fn check_for_player_movement(
    keyboard_input: &Res<Input<KeyCode>>,
    mut player_query: &mut Query<(
        &mut Player,
        (With<PlayerTile>, (Without<CollisionTile>, Without<Camera>)),
    )>,
    mut change_direction: &mut EventWriter<ChangeDirectionEvent>,
) {
    // Even though we don't change anything for the player, we need to use the mut().
    // Else we will get an ReadOnlyFetch error from the compiler
    if let Ok((mut player, _filters)) = player_query.single_mut() {
        // We only want to check if a key is pressed if we don't move from tile to tile
        if player.moving == Directions::None {
            if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
                change_direction.send(ChangeDirectionEvent(Directions::North));
            } else if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
                change_direction.send(ChangeDirectionEvent(Directions::South));
            } else if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
                change_direction.send(ChangeDirectionEvent(Directions::East));
            } else if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
                change_direction.send(ChangeDirectionEvent(Directions::West));
            }
        }
    }
}
