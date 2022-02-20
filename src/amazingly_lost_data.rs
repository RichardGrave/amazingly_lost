use crate::player;

use crate::maze_generator::{SMALL_MAZE, VERY_VERY_LARGE_MAZE};
use crate::maze_tile::MazeTile;
use crate::tile_factory::GameTileHandlers;
use bevy::ecs::prelude::Res;
use bevy::{asset::HandleId, prelude::*};
use player::Player;

pub const DEFAULT_THEME: &str = "default";

// To be sure we don't go over a certain size for the maze -> minus 100
pub const MAX_MAZE_SIZE_X_OR_Y: u16 = u16::MAX - 100;

//Important game
pub struct AmazinglyLostData {
    pub maze_size: (u16, u16),
    pub maze_solution: Vec<(usize, usize)>,
    pub starting_point_sprites: (f32, f32),
    pub exit_point_game: (usize, usize),
    pub player: Player,
    pub entity_player: Entity,
    // TODO:RG used settings??
    pub theme: String,
    pub game_tile_handlers: GameTileHandlers,
    pub is_generating_maze: bool,
    pub is_loading_assets: bool,
}

impl AmazinglyLostData {
    // Initialize game data
    pub fn new() -> Self {
        AmazinglyLostData {
            maze_size: (SMALL_MAZE, SMALL_MAZE),
            maze_solution: Vec::<(usize, usize)>::new(),
            starting_point_sprites: (0.0, 0.0),
            exit_point_game: (0, 0),
            player: Player::new(),
            entity_player: Entity::new(0),
            theme: DEFAULT_THEME.to_string(),
            game_tile_handlers: GameTileHandlers::new(),
            is_generating_maze: false,
            is_loading_assets: false,
        }
    }
}
