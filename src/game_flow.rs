use crate::amazingly_lost_data::AmazinglyLostData;
use crate::game_state::{ChangeGameStateEvent, GameState};
use crate::maze_generator::GameTile;
use crate::tile_factory::GameTileHandlers;
use crate::{maze_generator, tile_factory};
use bevy::render::camera::Camera;
use bevy::{asset::HandleId, prelude::*};

pub struct GameFlowPlugin;

impl Plugin for GameFlowPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(init_game_flow.system());
    }
}

fn init_game_flow(
    mut commands: Commands,
    mut amazing_data: ResMut<AmazinglyLostData>,
    mut camera_query: Query<(&mut Transform, &Camera)>,
    mut game_tile_query: Query<(Entity, (With<GameTile>, Without<Camera>))>,
    mut asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut game_state: ResMut<State<GameState>>,
    mut change_game_state: EventWriter<ChangeGameStateEvent>,
) {
    match game_state.current() {
        GameState::StartMenu => {}
        GameState::PlayingGame => {
            amazing_data.is_generating_maze = false;

            // TODO:RG
            // Do we need another State or some sub states for what is happening
            // during the game?
        }

        GameState::GenerateNewGame => {
            amazing_data.is_loading_assets = false;
            // Check if we are already generating.
            // Just in case there is a delay with Bevy and GeneratingGame is triggerd twice
            // This seems to happen often
            if !amazing_data.is_generating_maze {
                amazing_data.is_generating_maze = true;
                println!("GeneratingGame");
                // First clear the game field
                maze_generator::clear_maze_tiles(&mut commands, &mut game_tile_query);
                // Now create a new maze
                maze_generator::create_new_maze(
                    &mut commands,
                    &mut amazing_data,
                    &mut camera_query,
                );
                change_game_state.send(ChangeGameStateEvent(GameState::PlayingGame));
            } else {
                // println!("Tried to generate the maze twice");
            }
        }
        GameState::Settings => {
            println!("Settings");
        }
        GameState::Save => {
            println!("save");
        }
        GameState::LoadingAssets => {
            // Just in case there is a delay with Bevy and LoadingAssets is triggerd twice
            // This seems to happen often and we can counter this by using boolen checks
            if !amazing_data.is_loading_assets {
                amazing_data.is_loading_assets = true;
                tile_factory::load_all_assets(&mut amazing_data, &mut asset_server, &mut materials);
                change_game_state.send(ChangeGameStateEvent(GameState::GenerateNewGame));
            }
        }
    }
}
