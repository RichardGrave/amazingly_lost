use bevy::prelude::*;
mod amazingly_lost_data;
mod game_flow;
mod game_state;
mod game_ui;
mod keyboard_input;
mod maze_generator;
mod maze_tile;
mod player;
mod tile_factory;

use crate::game_flow::GameFlowPlugin;
use crate::game_state::{ChangeGameStateEvent, ChangeGameStatePlugin, GameState};
use crate::maze_generator::GameTile;
use crate::player::{ChangeDirectionEvent, ChangeDirectionPlugin};
use amazingly_lost_data::AmazinglyLostData;
use bevy::render::draw::OutsideFrustum;
use bevy::sprite::SpriteSettings;
use keyboard_input::KeyboardInputPlugin;
use player::Player;

const GAME_TITLE: &str = "Amazingly Lost";

fn main() {
    // Resources first
    App::build()
        .insert_resource(window_descriptor())
        .insert_resource(initialize_game_data())
        .insert_resource(ClearColor(Color::BLACK))
        // Enabling frustum culling can be removed if it's default in Bevy
        .insert_resource(SpriteSettings {
            frustum_culling_enabled: true,
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GameFlowPlugin)
        .add_plugin(ChangeGameStatePlugin)
        .add_plugin(ChangeDirectionPlugin)
        .add_plugin(KeyboardInputPlugin)
        .add_state(GameState::LoadingAssets)
        .add_event::<ChangeGameStateEvent>()
        .add_event::<ChangeDirectionEvent>()
        .add_startup_system(setup_game.system())
        .run();
}

//TODO:RG eventhandler

fn window_descriptor() -> WindowDescriptor {
    WindowDescriptor {
        title: GAME_TITLE.to_string(),
        width: 1440.0,
        height: 900.0,
        vsync: true,
        resizable: false,
        mode: bevy::window::WindowMode::Windowed,
        // TODO:RG use when finished
        // mode: bevy::window::WindowMode::BorderlessFullscreen,
        ..Default::default()
    }
}

//Initialize game data
fn initialize_game_data() -> AmazinglyLostData {
    AmazinglyLostData::new()
}

fn setup_game(mut commands: Commands, mut amazing_data: ResMut<AmazinglyLostData>) {
    // Camera for game menu's etc.
    // TODO:RG implement a use for this
    let mut ui_camera = UiCameraBundle::default();
    commands.spawn_bundle(ui_camera);

    // Camera for the game itself
    let mut camera = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(camera);
}
