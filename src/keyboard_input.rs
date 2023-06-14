use crate::player;
use crate::{amazingly_lost_data::AmazinglyLostData, player::Player};

use crate::game_state::{ChangeGameStateEvent, GameState};
use crate::maze_generator::{
    CollisionTile, GameTile, PlayerTile, SolutionTile, SMALL_MAZE, VERY_VERY_LARGE_MAZE,
};

use crate::player::ChangeDirectionEvent;

use bevy::app::AppExit;

use bevy::prelude::*;
use bevy::render::camera::Camera;
use bevy::window::WindowResized;
use player::Directions;

const MAZE_SIZE_SCALING: u16 = 33u16;

// For now we use the same value for frustum culling and zoom
const MAX_ZOOM_FRUSTUM: f32 = 25.0f32;
const MIN_ZOOM_FRUSTUM: f32 = 1.0f32;
const STEP_ZOOM_FRUSTUM: f32 = 1.0f32;

pub struct KeyboardInputPlugin;

impl Plugin for KeyboardInputPlugin {
    fn build(&self, app: &mut AppBuilder) {
        // We need a small input delay or the player will move to fast
        app.add_system(keyboard_input_game.system());
    }
}

pub fn keyboard_input_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_query: Query<(
        &mut Transform,
        (With<Camera>, (Without<CollisionTile>, Without<PlayerTile>)),
    )>,
    mut player_query: Query<(
        &mut Player,
        (With<PlayerTile>, (Without<CollisionTile>, Without<Camera>)),
    )>,
    mut mazetile_query: Query<(&mut Visible, (With<GameTile>, With<SolutionTile>))>,
    mut amazing_data: ResMut<AmazinglyLostData>,
    game_state: Res<State<GameState>>,
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
        } else if keyboard_input.just_pressed(KeyCode::P) {
            for (mut maze_tile_sprite, _) in mazetile_query.iter_mut() {
                // Hide current GameTiles and show Solution GameTiles or the other way around
                maze_tile_sprite.is_visible = !maze_tile_sprite.is_visible;
            }
        } else if keyboard_input.just_pressed(KeyCode::Q)
            || keyboard_input.just_pressed(KeyCode::Escape)
        {
            println!("Exit Game");
            exit.send(AppExit);
        } else if keyboard_input.just_pressed(KeyCode::PageUp) {
            if amazing_data.maze_size.0 < VERY_VERY_LARGE_MAZE {
                println!("Bigger maze");
                amazing_data.maze_size = (
                    amazing_data.maze_size.0 + MAZE_SIZE_SCALING,
                    amazing_data.maze_size.1 + MAZE_SIZE_SCALING,
                );
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
                amazing_data.maze_size = (
                    amazing_data.maze_size.0 - MAZE_SIZE_SCALING,
                    amazing_data.maze_size.1 - MAZE_SIZE_SCALING,
                );
                println!(
                    "SIZE: {:?}{:?}",
                    amazing_data.maze_size,
                    amazing_data.maze_size.0 as usize * amazing_data.maze_size.1 as usize
                );
                change_game_state.send(ChangeGameStateEvent(GameState::GenerateNewGame));
            }
        } else if keyboard_input.just_pressed(KeyCode::O) {
            // Zoom out
            for (mut transform, _) in camera_query.iter_mut() {
                if transform.scale.x + 1f32 < MAX_ZOOM_FRUSTUM {
                    transform.scale.x = transform.scale.x + STEP_ZOOM_FRUSTUM;
                    transform.scale.y = transform.scale.y + STEP_ZOOM_FRUSTUM;
                    transform.scale.z = transform.scale.z + STEP_ZOOM_FRUSTUM;
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::I) {
            // Zoom in
            for (mut transform, _) in camera_query.iter_mut() {
                if transform.scale.x - 1f32 >= MIN_ZOOM_FRUSTUM {
                    transform.scale.x = transform.scale.x - STEP_ZOOM_FRUSTUM;
                    transform.scale.y = transform.scale.y - STEP_ZOOM_FRUSTUM;
                    transform.scale.z = transform.scale.z - STEP_ZOOM_FRUSTUM;
                }
            }
        }
        // TODO:RG also option for free camera movement -> plus reset to player
    }
}

fn check_for_player_movement(
    keyboard_input: &Res<Input<KeyCode>>,
    player_query: &mut Query<(
        &mut Player,
        (With<PlayerTile>, (Without<CollisionTile>, Without<Camera>)),
    )>,
    change_direction: &mut EventWriter<ChangeDirectionEvent>,
) {
    // Even though we don't change anything for the player, we need to use the mut().
    // Else we will get an ReadOnlyFetch error from the compiler
    if let Ok((player, _filters)) = player_query.single_mut() {
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
