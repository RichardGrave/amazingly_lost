use crate::amazingly_lost_data::AmazinglyLostData;
use crate::game_state::GameState;
use crate::maze_tile::{MazeTile, TileType};
use crate::player::Player;
use crate::tile_factory::GameTileHandlers;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::render::camera::Camera;
use rand::Rng;
use std::{env, path::Path};

pub const SPRITE_SIZE_MAZE: usize = 100;
pub const NEXT_OPEN_WALL: usize = 15;
pub const DISTANCE_FROM_EXIT: usize = 10;

pub const SMALL_MAZE: u16 = 33u16;
pub const MEDIUM_MAZE: u16 = 77u16;
pub const LARGE_MAZE: u16 = 121u16;
pub const VERY_VERY_LARGE_MAZE: u16 = 231u16;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CollisionType {
    CollisionWall,
    CollisionExit,
    CollisionStart,
}

// Collision structs
#[derive(Clone, Debug, PartialEq)]
pub struct CollisionTile {
    pub collision_type: CollisionType,
}

// A MovingTile and LockedTile are both a GameTile
pub struct GameTile;
pub struct PlayerTile;

pub fn create_new_maze(
    mut commands: &mut Commands,
    mut amazing_data: &mut ResMut<AmazinglyLostData>,
    mut camera_query: &mut Query<(&mut Transform, &Camera)>,
) {
    let (mut maze, solution) = create_random_maze(
        amazing_data.maze_size.0 as usize,
        amazing_data.maze_size.1 as usize,
    );

    create_random_open_walls(&mut maze, &solution);

    // Make the maze visible for the player
    paint_the_maze(&mut commands, &solution, &mut maze, &mut amazing_data);

    place_camera_on_starting_tile(camera_query, &mut amazing_data);
    // Place player and camera on the same position

    place_player_in_maze(&mut commands, &mut amazing_data);

    // Set player starting position here. Is needed for camera/player placement
    let starting_point = *solution.first().unwrap();
    amazing_data.player.position_x = starting_point.0 as f32;
    amazing_data.player.position_y = starting_point.1 as f32;

    amazing_data.exit_point_game = *solution.last().unwrap();

    // set solution for this maze
    amazing_data.maze_solution = solution;
}

fn create_random_maze(width: usize, height: usize) -> (Vec<Vec<MazeTile>>, Vec<(usize, usize)>) {
    // Contains positions (x, y) we use this for random start point and later on
    // to save the start to end point solution
    let mut solution_path = Vec::<(usize, usize)>::new();

    // Use tmp_maze_path to get all usable starting points
    let (mut maze, mut pos_x, mut pos_y) = initialize_maze_size(&width, &height);

    // Starting position saved for solution
    solution_path.push((pos_x, pos_y));

    set_maze_tile_for_game(&mut pos_x, &mut pos_y, &mut maze, &mut solution_path);

    // Last tile is the exit
    let (last_x, last_y) = *solution_path.last().unwrap();
    maze[last_y][last_x].tile_type = TileType::Exit;

    (maze, solution_path)
}

// To make the maze a bit more difficult, we open up more walls so we have less obvious paths.
fn create_random_open_walls(maze: &mut Vec<Vec<MazeTile>>, solution_path: &Vec<(usize, usize)>) {
    let (exit_x, exit_y) = *solution_path.last().unwrap();
    let mut next_tile = NEXT_OPEN_WALL;

    loop {
        if next_tile < solution_path.len() {
            let (pos_x, pos_y) = *solution_path.get(next_tile).unwrap();

            // If we are to close to the exit then continue with another tile
            // Untill we are far enough from it
            if !((pos_y < exit_y && (exit_y - pos_y) > DISTANCE_FROM_EXIT)
                || (pos_y > exit_y && (pos_y - exit_y) > DISTANCE_FROM_EXIT))
                && !((pos_x < exit_x && (exit_x - pos_x) > DISTANCE_FROM_EXIT)
                    || (pos_x > exit_x && (pos_x - exit_x) > DISTANCE_FROM_EXIT))
            {
                next_tile += 1;
                continue;
            }

            // Wall North
            if maze[pos_y + 1][pos_x].tile_type == TileType::Wall
                // Only open walls that are not between a OPEN tile. No use to open it up
                && maze[pos_y + 1][pos_x - 1].tile_type == TileType::Wall
                && maze[pos_y + 1][pos_x + 1].tile_type == TileType::Wall
            {
                maze[pos_y + 1][pos_x].tile_type = TileType::Open;

                // Also check if the tile past this this wall is a wall.
                // Then we hit a T-junction and also need to open this wall
                if maze[pos_y + 2][pos_x].tile_type == TileType::Wall {
                    maze[pos_y + 2][pos_x].tile_type = TileType::Open;
                }

                // Wall South
            } else if maze[pos_y - 1][pos_x].tile_type == TileType::Wall
                // Only open walls that are not between a OPEN tile. No use to open it up
                && maze[pos_y - 1][pos_x - 1].tile_type == TileType::Wall
                && maze[pos_y - 1][pos_x + 1].tile_type == TileType::Wall
            {
                maze[pos_y - 1][pos_x].tile_type = TileType::Open;

                // Also check if the tile past this this wall is a wall.
                // Then we hit a T-junction
                if maze[pos_y - 2][pos_x].tile_type == TileType::Wall {
                    maze[pos_y - 2][pos_x].tile_type = TileType::Open;
                }

                // Wall East
            } else if maze[pos_y][pos_x + 1].tile_type == TileType::Wall
                // Only open walls that are not between a OPEN tile. No use to open it up
                && maze[pos_y - 1][pos_x + 1].tile_type == TileType::Wall
                && maze[pos_y + 1][pos_x + 1].tile_type == TileType::Wall
            {
                maze[pos_y][pos_x + 1].tile_type = TileType::Open;

                // Also check if the tile past this this wall is a wall.
                // Then we hit a T-junction
                if maze[pos_y][pos_x + 2].tile_type == TileType::Wall {
                    maze[pos_y][pos_x + 2].tile_type = TileType::Open;
                }

                // Wall West
            } else if maze[pos_y][pos_x - 1].tile_type == TileType::Wall
                // Only open walls that are not between a OPEN tile. No use to open it up
                && maze[pos_y - 1][pos_x - 1].tile_type == TileType::Wall
                && maze[pos_y + 1][pos_x - 1].tile_type == TileType::Wall
            {
                maze[pos_y][pos_x - 1].tile_type = TileType::Open;

                // Also check if the tile past this this wall is a wall.
                // Then we hit a T-junction
                if maze[pos_y][pos_x - 2].tile_type == TileType::Wall {
                    maze[pos_y][pos_x - 2].tile_type = TileType::Open;
                }

                // No wall to open, then find another wall at another tile
            } else {
                next_tile += 1;
                continue;
            }

            next_tile += NEXT_OPEN_WALL;
        } else {
            // Can't break open more walls
            break;
        }
    }
}

fn set_maze_tile_for_game(
    pos_x: &mut usize,
    pos_y: &mut usize,
    maze: &mut Vec<Vec<MazeTile>>,
    solution_path: &mut Vec<(usize, usize)>,
) {
    // But first make tmp_path equal to the solution_path so we have the starting point
    let mut tmp_path = Vec::<(usize, usize)>::new();
    tmp_path.clone_from(solution_path);

    loop {
        let (next_x, next_y, wall_x, wall_y) = get_random_directions(pos_x, pos_y, maze);

        if maze[next_y][next_x].id == maze[*pos_y][*pos_x].id {
            // In this case we can't continue to another tile and need to go back one step
            // one step in the tmp_maze_path and check that one for a new direction.
            if tmp_path.len() > 1 {
                // Remove last path tile
                tmp_path.pop().unwrap();
                // And another that contains the position of a WALL and can't be used for directions
                tmp_path.pop().unwrap();

                // Get last in list for new position to look at
                let last_x_y = tmp_path.last().unwrap(); // {
                *pos_x = last_x_y.0;
                *pos_y = last_x_y.1;
            } else {
                // This happens if we have used all the tiles
                // and only have the starting point in the tmp_path
                break;
            }
            continue;
        }

        maze[next_y][next_x].can_be_used = false;

        // Add WALL and next tile to the tmp_path
        tmp_path.push((wall_x, wall_y));
        tmp_path.push((next_x, next_y));
        *pos_x = next_x;
        *pos_y = next_y;

        // If tmp_path is longer than the solution then overwrite the solution
        // This way we get the longest path from start to end point
        if tmp_path.len() > solution_path.len() {
            solution_path.clone_from(&tmp_path);
        }
    }
}

fn get_random_directions(
    pos_x: &usize,
    pos_y: &usize,
    maze: &mut Vec<Vec<MazeTile>>,
) -> (usize, usize, usize, usize) {
    // Keep track of what direction we can use
    let mut directions = Vec::<(usize, usize, usize, usize)>::new();

    // Don't use &mut maze[y][x] because we would need it twice for current_tile and
    // the direction tile and that causes problem

    // If we go to a position that is a WALL, then we know that on the other side is
    // an OPEN tile. If the position is a BORDER, then on the other side is nothing

    // TODO:RG X and Y are switched

    if maze[*pos_y + 1][*pos_x].tile_type == TileType::Wall {
        // Check if we can use the tile on the other side
        if maze[*pos_y + 2][*pos_x].can_be_used {
            // Give the position of the WALL that can be set to OPEN
            // and give next tile position
            directions.push((*pos_x, *pos_y + 1, *pos_x, *pos_y + 2));
        }
    }

    if maze[*pos_y - 1][*pos_x].tile_type == TileType::Wall {
        // Check if we can use the tile on the other side
        if maze[*pos_y - 2][*pos_x].can_be_used {
            // Give the position of the WALL that can be set to OPEN
            // and give next tile position
            directions.push((*pos_x, *pos_y - 1, *pos_x, *pos_y - 2));
        }
    }

    if maze[*pos_y][*pos_x + 1].tile_type == TileType::Wall {
        // Check if we can use the tile on the other side
        if maze[*pos_y][*pos_x + 2].can_be_used {
            // Give the position of the WALL that can be set to OPEN
            // and give next tile position
            directions.push((*pos_x + 1, *pos_y, *pos_x + 2, *pos_y));
        }
    }

    if maze[*pos_y][*pos_x - 1].tile_type == TileType::Wall {
        // Check if we can use the tile on the other side
        if maze[*pos_y][*pos_x - 2].can_be_used {
            // Give the position of the WALL that can be set to OPEN
            // and give next tile position
            directions.push((*pos_x - 1, *pos_y, *pos_x - 2, *pos_y));
        }
    }

    get_new_direction(pos_x, pos_y, &directions, maze)
}

fn get_new_direction(
    pos_x: &usize,
    pos_y: &usize,
    directions: &Vec<(usize, usize, usize, usize)>,
    maze: &mut Vec<Vec<MazeTile>>,
) -> (usize, usize, usize, usize) {
    // We can have none, one or multiple directions

    // Default direction is our current tile and 0, 0 is not used
    // and needed if we can't go in other directions
    let mut direction = &(0, 0, *pos_x, *pos_y);

    if directions.len() == 1 {
        // Only one direction to use
        direction = directions.first().unwrap();
    } else if directions.len() > 1 {
        // We have multiple directions, so we need to get a random tile
        let random_direction = rand::thread_rng().gen_range(0..directions.len());
        direction = directions.get(random_direction).unwrap();
    }

    let wall_x = direction.0;
    let wall_y = direction.1;
    let next_x = direction.2;
    let next_y = direction.3;

    // Doensn't matter if this is the current tile or a WALL.
    // But position can't be 0,0 for a wall
    if wall_x != 0 && wall_y != 0 {
        maze[wall_y][wall_x].tile_type = TileType::Open;
    }

    // Return next position
    (next_x, next_y, wall_x, wall_y)
}

// Create an maze with default value tiles
fn initialize_maze_size(width: &usize, height: &usize) -> (Vec<Vec<MazeTile>>, usize, usize) {
    let mut tmp_open_tiles = Vec::<(usize, usize)>::new();

    let mut maze = Vec::<Vec<MazeTile>>::new();
    let mut tile_id = 0;

    for y in 0..*height {
        let mut maze_row = Vec::<MazeTile>::new();

        for x in 0..*width {
            // Start at id 1
            tile_id += 1;

            // This is always a BORDER
            if y == 0 || y == *height - 1 || x == 0 || x == *width - 1 {
                // Borders can never change to OPEN
                maze_row.push(MazeTile::new(tile_id, TileType::Border));

                // Every odd position is an OPEN TileType
            } else if x % 2 != 0 && y % 2 != 0 {
                // Add OPEN tile tmp_open_tiles for random startpoint
                tmp_open_tiles.push((x, y));
                maze_row.push(MazeTile::new(tile_id, TileType::Open));

                // The rest are all walls
            } else {
                maze_row.push(MazeTile::new(tile_id, TileType::Wall));
            }
        }
        maze.push(maze_row);
    }
    // Find a starting point
    let random_number = rand::thread_rng().gen_range(0..tmp_open_tiles.len() - 1);
    let random_position = tmp_open_tiles.get(random_number).unwrap();

    maze[random_position.1][random_position.0].tile_type = TileType::Start;
    maze[random_position.1][random_position.0].can_be_used = false;

    (maze, random_position.0, random_position.1)
}

pub fn paint_the_maze(
    mut commands: &mut Commands,
    solution: &Vec<(usize, usize)>,
    maze: &mut Vec<Vec<MazeTile>>,
    mut amazing_data: &mut ResMut<AmazinglyLostData>,
) {
    // TODO:RG WALLS also part of solution and dont overpaint
    for (x, y) in solution {
        maze[*y][*x].part_of_solution = true;
    }

    // z-axis is always 0.0, we don't use depth

    let mut pos_y = SPRITE_SIZE_MAZE as f32; // / 2.0;

    for maze_row in maze {
        let mut pos_x = SPRITE_SIZE_MAZE as f32; // / 2.0;

        for maze_tile in maze_row {
            match &maze_tile.tile_type {
                TileType::Border | TileType::Wall => {
                    if let Some(mut border_texuture_handle) =
                        amazing_data.game_tile_handlers.get_random_game_tile(
                            &maze_tile.tile_type,
                            &Vec3::new(pos_x, pos_y as f32, 1.0),
                        )
                    {
                        border_texuture_handle.sprite = Sprite::new(Vec2::new(
                            SPRITE_SIZE_MAZE as f32,
                            SPRITE_SIZE_MAZE as f32,
                        ));

                        commands
                            .spawn_bundle(border_texuture_handle.clone())
                            .insert(GameTile)
                            .insert(CollisionTile {
                                collision_type: CollisionType::CollisionWall,
                            });
                    }
                }
                TileType::Start => {
                    println!("Start:{}-{}", pos_x, pos_y);
                    amazing_data.starting_point_sprites = (pos_x, pos_y);
                    if let Some(mut start_texuture_handle) = amazing_data
                        .game_tile_handlers
                        .get_game_start(&Vec3::new(pos_x, pos_y, 1.0))
                    {
                        start_texuture_handle.sprite = Sprite::new(Vec2::new(
                            SPRITE_SIZE_MAZE as f32,
                            SPRITE_SIZE_MAZE as f32,
                        ));

                        commands
                            .spawn_bundle(start_texuture_handle.clone())
                            .insert(GameTile)
                            .insert(CollisionTile {
                                collision_type: CollisionType::CollisionStart,
                            });
                    }
                }
                TileType::Exit => {
                    if let Some(mut exit_texuture_handle) = amazing_data
                        .game_tile_handlers
                        .get_game_exit(&Vec3::new(pos_x, pos_y, 1.0))
                    {
                        exit_texuture_handle.sprite = Sprite::new(Vec2::new(
                            SPRITE_SIZE_MAZE as f32,
                            SPRITE_SIZE_MAZE as f32,
                        ));

                        commands
                            .spawn_bundle(exit_texuture_handle.clone())
                            .insert(GameTile)
                            .insert(CollisionTile {
                                collision_type: CollisionType::CollisionExit,
                            });
                    }
                }
                TileType::Open => {
                    if let Some(mut open_texuture_handle) = amazing_data
                        .game_tile_handlers
                        .get_random_game_tile(&maze_tile.tile_type, &Vec3::new(pos_x, pos_y, 1.0))
                    {
                        open_texuture_handle.sprite = Sprite::new(Vec2::new(
                            SPRITE_SIZE_MAZE as f32,
                            SPRITE_SIZE_MAZE as f32,
                        ));

                        commands
                            .spawn_bundle(open_texuture_handle.clone())
                            .insert(GameTile);
                    }
                }
            }

            pos_x += SPRITE_SIZE_MAZE as f32;
        }
        pos_y += SPRITE_SIZE_MAZE as f32;
    }
}

fn place_player_in_maze(commands: &mut Commands, amazing_data: &mut ResMut<AmazinglyLostData>) {
    if let Some(mut player_texture_handle) =
        amazing_data.game_tile_handlers.get_game_player(&Vec3::new(
            // Place the player at the starting point
            amazing_data.starting_point_sprites.0 as f32,
            amazing_data.starting_point_sprites.1 as f32,
            1.0,
        ))
    {
        player_texture_handle.sprite =
            Sprite::new(Vec2::new(SPRITE_SIZE_MAZE as f32, SPRITE_SIZE_MAZE as f32));
        amazing_data.entity_player = commands
            .spawn_bundle(player_texture_handle)
            .insert(GameTile)
            .insert(PlayerTile)
            .insert(amazing_data.player.clone())
            .id();
    }
}

fn place_camera_on_starting_tile(
    mut camera_query: &mut Query<(&mut Transform, &Camera)>,
    mut amazing_data: &mut ResMut<AmazinglyLostData>,
) {
    for mut camera in camera_query.iter_mut() {
        // Place Camera on the same x, y as the Player
        camera.0.translation.x = amazing_data.starting_point_sprites.0 as f32;
        camera.0.translation.y = amazing_data.starting_point_sprites.1 as f32;
    }
}

pub fn clear_maze_tiles(
    mut commands: &mut Commands,
    mut game_tile_query: &mut Query<(Entity, (With<GameTile>, Without<Camera>))>,
) {
    // Despawn every GameTile, but NOT the camera. Or we will see a black screen
    for (entity, _game_tile) in game_tile_query.iter_mut() {
        commands.entity(entity).despawn();
    }
}
