use bevy::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    Start,
    Exit,
    Wall,
    Border,
    Open,
}

// impl<T> PartialEq<T> for TileType {
//     fn eq(&self, other: &Self) -> bool {
//         self == other
//     }
// }

#[derive(Clone, Debug, PartialEq)]
pub struct MazeTile {
    // This is set to false if we have used it during maze generation or is a permanent WALL
    // So we can easly see if we can go to this one from another MazeTile the generation.
    pub can_be_used: bool,

    pub id: usize,
    pub tile_type: TileType,
    pub pos_x: f32,
    pub pos_y: f32,
    pub part_of_solution: bool,
}

impl MazeTile {
    pub fn new(id: usize, tile_type: TileType) -> Self {
        Self {
            can_be_used: tile_type != TileType::Wall && tile_type != TileType::Border,
            id,
            tile_type,
            pos_x: 0.0,
            pos_y: 0.0,
            part_of_solution: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MazeRegion {
    pub id: usize,
    pub pos_x: f32,
    pub pos_y: f32,
    pub region_maze_tiles: Vec<MazeTile>,
}

impl MazeRegion {
    pub fn new(id: usize, pos_x: f32, pos_y: f32) -> Self {
        Self {
            id,
            pos_x,
            pos_y,
            region_maze_tiles: Vec::<MazeTile>::new(),
        }
    }
}
