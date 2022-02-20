use bevy::{asset::HandleId, prelude::*};
use rand::Rng;
use std::path::Path;
use std::{env, path::PathBuf};

use crate::amazingly_lost_data::AmazinglyLostData;
use crate::game_state::GameState;
use crate::maze_tile::TileType;

// Paths to the textures
pub const WALLS_NORMAL: &str = "walls/normal/wall_";
pub const WALLS_SPECIAL: &str = "walls/special/wall_";
pub const GROUNDS_NORMAL: &str = "grounds/normal/ground_";
pub const GROUNDS_SPECIAL: &str = "grounds/special/ground_";

// Max per ground, wall -> normal, special
pub const MAX_ASSETS: u8 = 50;

// Assets and their probability of being randomly selected
// in % for walls
pub const MAX_CHANCE_WALLS: u8 = 100;
// 100%
pub const CHANCE_SPECIAL_WALL: u8 = 5;
// 5%
pub const CHANCE_NORMAL_WALL: u8 = MAX_CHANCE_WALLS - CHANCE_SPECIAL_WALL; // Should be 100 - 5 = 95%

pub const MAX_CHANCE_BORDER: u8 = 100;
// 100%
pub const CHANCE_SPECIAL_BORDER: u8 = 5;
// 5%
pub const CHANCE_NORMAL_BORDER: u8 = MAX_CHANCE_BORDER - CHANCE_SPECIAL_BORDER; // Should be 100 - 5 = 95%

//for grounds
pub const MAX_CHANCE_GROUNDS: u8 = 100;
// 100%
pub const CHANCE_SPECIAL_GROUND: u8 = 5;
// 5%
pub const CHANCE_NORMAL_GROUND: u8 = MAX_CHANCE_GROUNDS - CHANCE_SPECIAL_GROUND; // Should be 100 - 5 = 95%

// pub struct GameTileFactoryPlugin;
//
// impl Plugin for GameTileFactoryPlugin {
//     fn build(&self, app: &mut AppBuilder) {
//         app.insert_resource(initialize_game_components())
//             .add_startup_system(load_all_assets.system());
//     }
// }

// #[derive(Clone, PartialEq)]
pub struct GameTileHandlers {
    // Multiple walls for randomness
    pub normal_walls: Vec<Handle<ColorMaterial>>,
    pub special_walls: Vec<Handle<ColorMaterial>>,
    // Multiple open places for randomness
    pub normal_grounds: Vec<Handle<ColorMaterial>>,
    pub special_grounds: Vec<Handle<ColorMaterial>>,
    // Unique tiles
    pub player: Handle<ColorMaterial>,
    pub start: Handle<ColorMaterial>,
    pub exit: Handle<ColorMaterial>,
    pub region: Handle<ColorMaterial>,
}

impl GameTileHandlers {
    pub fn new() -> Self {
        Self {
            normal_walls: Vec::<Handle<ColorMaterial>>::new(),
            // Should contain funny walls or something like that.
            special_walls: Vec::<Handle<ColorMaterial>>::new(),

            normal_grounds: Vec::<Handle<ColorMaterial>>::new(),
            // Should contain funny grounds or something like that.
            special_grounds: Vec::<Handle<ColorMaterial>>::new(),
            // We should have only a unique player, start and exit tile
            // So, no list
            player: Handle::weak(HandleId::default::<ColorMaterial>()),
            start: Handle::weak(HandleId::default::<ColorMaterial>()),
            exit: Handle::weak(HandleId::default::<ColorMaterial>()),
            region: Handle::weak(HandleId::default::<ColorMaterial>()),
        }
    }

    pub fn get_random_game_tile(
        &self,
        tile_type: &TileType,
        position: &Vec3,
    ) -> Option<SpriteBundle> {
        // +1 because we start at 1
        let random_number = rand::thread_rng().gen_range(1..=MAX_CHANCE_WALLS);

        match tile_type {
            TileType::Wall => match random_number {
                // Highest chance to get a normal wall
                1..=CHANCE_NORMAL_WALL => self.get_game_tile(tile_type, true, position),
                // Special walls should be so special that we don't want too many of them
                _ => self.get_game_tile(tile_type, false, position),
            },
            TileType::Border => match random_number {
                // Highest chance to get a normal wall
                1..=CHANCE_NORMAL_BORDER => self.get_game_tile(tile_type, true, position),
                // Special walls should be so special that we don't want too many of them
                _ => self.get_game_tile(tile_type, false, position),
            },
            TileType::Open => match random_number {
                // Highest chance to get a normal wall
                1..=CHANCE_NORMAL_GROUND => self.get_game_tile(tile_type, true, position),
                // Special walls should be so special that we don't want too many of them
                _ => self.get_game_tile(tile_type, false, position),
            },
            _ => None,
        }
    }

    fn get_game_tile(
        &self,
        tile_type: &TileType,
        is_normal: bool,
        position: &Vec3,
    ) -> Option<SpriteBundle> {
        if let Some(handle) = self.get_handle(tile_type, is_normal) {
            Some(SpriteBundle {
                material: handle.clone(),
                transform: Transform::from_translation(*position),
                ..Default::default()
            })
        } else {
            None
        }
    }

    pub fn get_game_player(&self, position: &Vec3) -> Option<SpriteBundle> {
        if let handle = &self.player {
            Some(SpriteBundle {
                material: handle.clone(),
                transform: Transform::from_translation(*position),
                ..Default::default()
            })
        } else {
            None
        }
    }

    pub fn get_game_start(&self, position: &Vec3) -> Option<SpriteBundle> {
        if let handle = &self.start {
            Some(SpriteBundle {
                material: handle.clone(),
                transform: Transform::from_translation(*position),
                ..Default::default()
            })
        } else {
            None
        }
    }

    pub fn get_game_exit(&self, position: &Vec3) -> Option<SpriteBundle> {
        if let handle = &self.exit {
            Some(SpriteBundle {
                material: handle.clone(),
                transform: Transform::from_translation(*position),
                ..Default::default()
            })
        } else {
            None
        }
    }

    pub fn get_game_region(&self, position: &Vec3) -> Option<SpriteBundle> {
        if let handle = &self.region {
            Some(SpriteBundle {
                material: handle.clone(),
                transform: Transform::from_translation(*position),
                ..Default::default()
            })
        } else {
            None
        }
    }

    fn get_handle(&self, tile_type: &TileType, is_normal: bool) -> Option<&Handle<ColorMaterial>> {
        match tile_type {
            TileType::Start => self.get_ground(is_normal),
            TileType::Exit => self.get_a_wall(is_normal),
            TileType::Wall => self.get_a_wall(is_normal),
            TileType::Border => self.get_a_wall(is_normal),
            TileType::Open => self.get_ground(is_normal),
        }
    }

    fn get_a_wall(&self, is_normal: bool) -> Option<&Handle<ColorMaterial>> {
        if is_normal {
            if self.normal_walls.len() > 0 {
                self.normal_walls
                    .get(rand::thread_rng().gen_range(0..self.normal_walls.len()))
            } else {
                // If there is no normal wall then don't go further
                None
            }
        } else {
            if self.special_walls.len() > 0 {
                self.special_walls
                    .get(rand::thread_rng().gen_range(0..self.special_walls.len()))
            } else {
                // If there are no special walls, then try to get a normal
                self.get_a_wall(true)
            }
        }
    }

    fn get_ground(&self, is_normal: bool) -> Option<&Handle<ColorMaterial>> {
        if is_normal {
            if self.normal_grounds.len() > 0 {
                self.normal_grounds
                    .get(rand::thread_rng().gen_range(0..self.normal_grounds.len()))
            } else {
                // If there is no normal ground then don't go further
                None
            }
        } else {
            if self.special_grounds.len() > 0 {
                self.special_grounds
                    .get(rand::thread_rng().gen_range(0..self.special_grounds.len()))
            } else {
                // If there are no special grounds, then try to get a normal
                self.get_ground(true)
            }
        }
    }
}

pub fn load_all_assets(
    mut amazing_data: &mut ResMut<AmazinglyLostData>,
    mut asset_server: &mut Res<AssetServer>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let current_dir = env::current_dir().unwrap();

    let tile_theme = &amazing_data.theme.clone();
    // Walls
    amazing_data.game_tile_handlers.normal_walls = load_game_tiles(
        &current_dir,
        &mut asset_server,
        &mut materials,
        &tile_theme,
        &WALLS_NORMAL.to_string(),
    );
    amazing_data.game_tile_handlers.special_walls = load_game_tiles(
        &current_dir,
        &mut asset_server,
        &mut materials,
        &tile_theme,
        &WALLS_SPECIAL.to_string(),
    );
    // Grounds
    amazing_data.game_tile_handlers.normal_grounds = load_game_tiles(
        &current_dir,
        &mut asset_server,
        &mut materials,
        &tile_theme,
        &GROUNDS_NORMAL.to_string(),
    );
    amazing_data.game_tile_handlers.special_grounds = load_game_tiles(
        &current_dir,
        &mut asset_server,
        &mut materials,
        &tile_theme,
        &GROUNDS_SPECIAL.to_string(),
    );

    load_game_uniques(
        &current_dir,
        &mut asset_server,
        &mut materials,
        &mut amazing_data,
    );
}

fn load_game_tiles(
    current_dir: &PathBuf,
    mut asset_server: &mut Res<AssetServer>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    theme: &String,
    tile_dir: &String,
) -> Vec<Handle<ColorMaterial>> {
    let mut game_tiles_vec = Vec::<Handle<ColorMaterial>>::new();

    // I don't want to have to many game tiles per wall, ground, specials etc.
    for x in 1..MAX_ASSETS {
        let png_file = format!(
            "{}/assets/theme/{}/{}{}.png",
            current_dir.display(),
            theme,
            tile_dir,
            x
        );
        // println!("path: {}", png_file);
        // Only if found, so don't skip in numbers.
        // !!! E.g. wall_1, wall_2 and suddenly wall_4. In this case we also need wall_3
        if Path::new(png_file.as_str()).exists() {
            // println!("Path exits");
            // Add it to the walls vec for later use
            game_tiles_vec.push(materials.add(asset_server.load(png_file.as_str()).clone().into()));
        } else {
            // Just in case we don't find any Wallw or Grounds
            if game_tiles_vec.len() == 0 {
                if *tile_dir == WALLS_NORMAL.to_string() {
                    game_tiles_vec.push(materials.add(Color::DARK_GREEN.into()));
                } else if *tile_dir == GROUNDS_NORMAL.to_string() {
                    game_tiles_vec.push(materials.add(Color::LIME_GREEN.into()));
                }
            }
            // println!("Path DOESN'T exits");
            // If we don't find more walls, then stop
            break;
        }
    }
    game_tiles_vec
}

// TODO:RG for now this works, but what if i wan't animations???
// We only have one of a player, start and exit. No need to do a loop.
fn load_game_uniques(
    current_dir: &PathBuf,
    mut asset_server: &mut Res<AssetServer>,
    mut materials: &mut ResMut<Assets<ColorMaterial>>,
    mut amazing_data: &mut ResMut<AmazinglyLostData>,
) {
    let start_png_file = format!(
        "{}/assets/theme/{}/uniques/start.png",
        current_dir.display(),
        amazing_data.theme
    );
    // println!("path: {}", start_png_file);
    let exit_png_file = format!(
        "{}/assets/theme/{}/uniques/exit.png",
        current_dir.display(),
        amazing_data.theme
    );
    // println!("path: {}", exit_png_file);
    let player_png_file = format!(
        "{}/assets/theme/{}/uniques/player.png",
        current_dir.display(),
        amazing_data.theme
    );
    // println!("path: {}", player_png_file);

    if Path::new(start_png_file.as_str()).exists() {
        // println!("Path exits");
        amazing_data.game_tile_handlers.start =
            materials.add(asset_server.load(start_png_file.as_str()).clone().into());
    } else {
        // println!("Path DOESN'T exits");
        amazing_data.game_tile_handlers.start = materials.add(Color::PURPLE.into());
    }

    if Path::new(exit_png_file.as_str()).exists() {
        // println!("Path exits");
        amazing_data.game_tile_handlers.exit =
            materials.add(asset_server.load(exit_png_file.as_str()).clone().into());
    } else {
        // println!("Path DOESN'T exits");
        amazing_data.game_tile_handlers.exit = materials.add(Color::BLUE.into());
    }

    if Path::new(player_png_file.as_str()).exists() {
        // println!("Path exits");
        amazing_data.game_tile_handlers.player =
            materials.add(asset_server.load(player_png_file.as_str()).clone().into());
    } else {
        // println!("Path DOESN'T exits");
        amazing_data.game_tile_handlers.player = materials.add(Color::YELLOW.into());
    }
}
