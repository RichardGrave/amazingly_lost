use crate::player;
use crate::{amazingly_lost_data::AmazinglyLostData, player::Player};

use crate::game_state::{ChangeGameStateEvent, GameState};
use crate::maze_generator::{CollisionTile, PlayerTile, SMALL_MAZE, VERY_VERY_LARGE_MAZE};

use crate::player::ChangeDirectionEvent;

use bevy::app::AppExit;

use bevy::prelude::*;
use bevy::render::camera::{Camera, OrthographicProjection};
use bevy::sprite::SpriteSettings;
use bevy::window::WindowResized;
use player::Directions;

const MAZE_SIZE_SCALING: u16 = 33u16;

// For now we use the same value for frustum culling and zoom
const MAX_ZOOM_FRUSTUM: f32 = 10.0f32;
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
    windows: Res<Windows>,
    // active_cameras: Res<ActiveCameras>,
    mut sprite_settings: ResMut<SpriteSettings>,
    mut camera_query: Query<(
        &mut OrthographicProjection,
        &Camera,
        &mut Transform,
        (With<Camera>, (Without<CollisionTile>, Without<PlayerTile>)),
    )>,
    mut player_query: Query<(
        &mut Player,
        (With<PlayerTile>, (Without<CollisionTile>, Without<Camera>)),
    )>,
    mut amazing_data: ResMut<AmazinglyLostData>,
    game_state: Res<State<GameState>>,
    mut change_game_state: EventWriter<ChangeGameStateEvent>,
    mut change_direction: EventWriter<ChangeDirectionEvent>,
    mut changed_window: EventWriter<WindowResized>,
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
        } else if keyboard_input.just_pressed(KeyCode::U) {
            for (mut ortho_projection, camera, mut _transform, _) in camera_query.iter_mut() {
                if ortho_projection.scale + 1f32 < MAX_ZOOM_FRUSTUM {
                    ortho_projection.scale = ortho_projection.scale + STEP_ZOOM_FRUSTUM;

                    // We need to send a WindowResized event or it wil not zoom
                    if let Some(game_window) = windows.get(camera.window) {
                        changed_window.send(WindowResized {
                            id: camera.window,
                            width: game_window.width(),
                            height: game_window.height(),
                        });
                    }
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::I) {
            for (mut ortho_projection, camera, mut _transform, _) in camera_query.iter_mut() {
                if ortho_projection.scale - 1f32 >= MIN_ZOOM_FRUSTUM {
                    ortho_projection.scale = ortho_projection.scale - STEP_ZOOM_FRUSTUM;

                    // We need to send a WindowResized event or it wil not zoom
                    if let Some(game_window) = windows.get(camera.window) {
                        changed_window.send(WindowResized {
                            id: camera.window,
                            width: game_window.width(),
                            height: game_window.height(),
                        });
                    }
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::J) {
            for (mut _ortho_projection, _camera, mut transform, _) in camera_query.iter_mut() {
                // TODO:RG does frustum culling, but this also seems to zoom -> found out if this can be prevented
                if transform.scale.x + 1f32 < MAX_ZOOM_FRUSTUM {
                    transform.scale.x = transform.scale.x + STEP_ZOOM_FRUSTUM;
                    transform.scale.y = transform.scale.y + STEP_ZOOM_FRUSTUM;
                    transform.scale.z = transform.scale.z + STEP_ZOOM_FRUSTUM;
                }
            }
        } else if keyboard_input.just_pressed(KeyCode::K) {
            for (mut _ortho_projection, _camera, mut transform, _) in camera_query.iter_mut() {
                // TODO:RG does frustum culling, but this also seems to zoom -> found out if this can be prevented
                if transform.scale.x - 1f32 >= MIN_ZOOM_FRUSTUM {
                    transform.scale.x = transform.scale.x - STEP_ZOOM_FRUSTUM;
                    transform.scale.y = transform.scale.y - STEP_ZOOM_FRUSTUM;
                    transform.scale.z = transform.scale.z - STEP_ZOOM_FRUSTUM;
                }
            }
        }
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
