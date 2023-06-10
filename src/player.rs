use crate::game_state::{ChangeGameStateEvent, GameState};
use crate::maze_generator::{CollisionTile, CollisionType, PlayerTile, SPRITE_SIZE_MAZE};

use bevy::render::camera::Camera;
use bevy::render::draw::OutsideFrustum;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
};

// SPRITE_SIZE_MAZE should always be evenly divisable by the MOVEMENT_ACCELERATION
// SPRITE_SIZE_MAZE 100 and MOVEMENT_ACCELERATION 20.0 seems the best to use
pub const MOVEMENT_ACCELERATION: f32 = 20.0f32;
pub const MOVEMENT: f32 = SPRITE_SIZE_MAZE as f32 / MOVEMENT_ACCELERATION; //This was default -> 3.0f32;

#[derive(Clone, Debug, Eq, PartialEq, Copy)]
pub enum Directions {
    North,
    South,
    East,
    West,
    None,
}

pub struct CollisionWith {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    exit: bool,
}

impl CollisionWith {
    pub fn new() -> Self {
        Self {
            left: false,
            right: false,
            up: false,
            down: false,
            exit: false,
        }
    }
}

pub struct ChangeDirectionEvent(pub Directions);

pub struct ChangeDirectionPlugin;

impl Plugin for ChangeDirectionPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(check_direction_change.system());
    }
}

pub fn check_direction_change(
    mut camera_query: Query<(
        &mut Transform,
        (With<Camera>, (Without<CollisionTile>, Without<PlayerTile>)),
    )>,
    mut player_query: Query<(
        &mut Transform,
        &mut Sprite,
        &mut Player,
        (With<PlayerTile>, (Without<CollisionTile>, Without<Camera>)),
    )>,
    mut collision_query: Query<(
        &Transform,
        &Sprite,
        &CollisionTile,
        (
            With<CollisionTile>,
            (Without<Camera>, Without<OutsideFrustum>),
        ),
    )>,
    mut change_direction: EventReader<ChangeDirectionEvent>,
    mut change_game_state: EventWriter<ChangeGameStateEvent>,
    game_state: ResMut<State<GameState>>,
) {
    let mut new_direction = Directions::None;

    for direction_event in change_direction.iter() {
        new_direction = direction_event.0;
    }

    move_to_next_maze_tile(
        &mut camera_query,
        &mut player_query,
        &mut collision_query,
        &mut change_game_state,
        &new_direction,
        &game_state.current(),
    );
}

#[derive(Clone)]
pub struct Player {
    pub position_x: f32,
    pub position_y: f32,
    pub moving: Directions,
    // TODO:RG Maybe items that can be used to help. Show maze_solution, flares if it's dark, etc.
    pub direction: Directions,
    pub next_position_x: f32,
    pub next_position_y: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            position_x: 0.0,
            position_y: 0.0,
            moving: Directions::None,
            direction: Directions::North,
            next_position_x: 0.0,
            next_position_y: 0.0,
        }
    }
}

pub fn move_to_next_maze_tile(
    camera_query: &mut Query<(
        &mut Transform,
        (With<Camera>, (Without<CollisionTile>, Without<PlayerTile>)),
    )>,
    player_query: &mut Query<(
        &mut Transform,
        &mut Sprite,
        &mut Player,
        (With<PlayerTile>, (Without<CollisionTile>, Without<Camera>)),
    )>,
    collision_query: &mut Query<(
        &Transform,
        &Sprite,
        &CollisionTile,
        (
            With<CollisionTile>,
            (Without<Camera>, Without<OutsideFrustum>),
        ),
    )>,
    change_game_state: &mut EventWriter<ChangeGameStateEvent>,
    new_direction: &Directions,
    game_state: &GameState,
) {
    // Only usefull while playing the game
    if *game_state == GameState::PlayingGame {
        if let Ok((mut player_transform, mut player_sprite, mut player, _player_tile)) =
            player_query.single_mut()
        {
            if *new_direction != Directions::None && player.moving == Directions::None {
                let mut collision_with = CollisionWith::new();
                let mut collision_count = 0;
                // check collision with walls
                for (collider_transform, collider_sprite, collider, _filter) in
                    collision_query.iter_mut()
                {
                    collision_count += 1;

                    // TODO:RG do collision check in separate function
                    //        function is getting to big

                    // TODO:RG also change facing texture of player for left and right

                    // Check for collisions before we move, to prevent getting stuck in a wall
                    let collision = check_for_collision(
                        new_direction,
                        &collider_transform.translation,
                        &collider_sprite.size,
                        &player_transform.translation,
                        &mut player_sprite,
                    );

                    if let Some(collision_side) = collision {
                        // println!("Collider: {:?}", collider);
                        if collider.collision_type == CollisionType::CollisionWall {
                            match collision_side {
                                Collision::Left => collision_with.left = true,
                                Collision::Right => collision_with.right = true,
                                Collision::Top => collision_with.up = true,
                                Collision::Bottom => collision_with.down = true,
                            }
                        } else if collider.collision_type == CollisionType::CollisionExit {
                            collision_with.exit = true;
                        }
                    }
                }
                if !collision_with.left
                    && !collision_with.right
                    && !collision_with.up
                    && !collision_with.down
                    && !collision_with.exit
                {
                    // Set new coordinates for the player
                    set_next_player_position(
                        new_direction,
                        &mut player,
                        &mut player_transform.translation,
                    );
                } else if collision_with.exit {
                    println!("EXIT");
                    // TODO:RG for now we generate a new maze
                    change_game_state.send(ChangeGameStateEvent(GameState::GenerateNewGame));
                }
            } else {
                // println!("Collisino size: {:?}", collision_count);
                // Update player position until we get to the new coordinates
                if player.moving != Directions::None {
                    handle_movement_by_collision(
                        &mut player,
                        &mut player_transform.translation,
                        camera_query,
                    );
                }
            }
        }
    }
}

fn set_next_player_position(
    new_direction: &Directions,
    player: &mut Player,
    player_translation: &mut Vec3,
) {
    if *new_direction == Directions::West {
        // player_translation.x -= MOVEMENT;
        player.next_position_x = player_translation.x - SPRITE_SIZE_MAZE as f32;
        player.next_position_y = player_translation.y;
        player.moving = Directions::West;
        println!("Go left");
    } else if *new_direction == Directions::East {
        // player_translation.x += MOVEMENT;
        player.next_position_x = player_translation.x + SPRITE_SIZE_MAZE as f32;
        player.next_position_y = player_translation.y;
        player.moving = Directions::East;
        println!("Go right");
    } else if *new_direction == Directions::North {
        // player_translation.y += MOVEMENT;
        player.next_position_y = player_translation.y + SPRITE_SIZE_MAZE as f32;
        player.next_position_x = player_translation.x;
        player.moving = Directions::North;
        println!("Go up");
    } else if *new_direction == Directions::South {
        // player_translation.y -= MOVEMENT;
        player.next_position_y = player_translation.y - SPRITE_SIZE_MAZE as f32;
        player.next_position_x = player_translation.x;
        player.moving = Directions::South;
        println!("Go down");
    }
    // println!("Direction: {:?} - posx: {:?} - posy:{:?}", new_direction, player.next_position_x, player.next_position_y);
}

fn handle_movement_by_collision(
    player: &mut Player,
    player_translation: &mut Vec3,
    camera_query: &mut Query<(
        &mut Transform,
        (With<Camera>, (Without<CollisionTile>, Without<PlayerTile>)),
    )>,
) {
    if player.moving == Directions::West {
        player_translation.x -= MOVEMENT;

        // println!("Move left");
    } else if player.moving == Directions::East {
        player_translation.x += MOVEMENT;

        // println!("Move right");
    } else if player.moving == Directions::North {
        player_translation.y += MOVEMENT;

        // println!("Move up");
    } else if player.moving == Directions::South {
        player_translation.y -= MOVEMENT;

        // println!("Move down");
    }

    if player_translation.x == player.next_position_x
        && player_translation.y == player.next_position_y
    {
        // println!("Finished moving");
        player.moving = Directions::None;
        player.position_x = player.next_position_x;
        player.position_y = player.next_position_y;
    }
    // Keep camera's on the same position as the player
    for (mut camera_transform, _camera) in camera_query.iter_mut() {
        camera_transform.translation = *player_translation;
    }
}

fn check_for_collision(
    new_direction: &Directions,
    collider_translation: &Vec3,
    collider_size: &Vec2,
    player_translation: &Vec3,
    // player_size: &Vec2,
    player_sprite: &mut Sprite,
) -> Option<Collision> {
    let mut next_player_translation = (*player_translation).clone();

    if *new_direction == Directions::West {
        next_player_translation.x -= MOVEMENT;
        // Also set player Sprite to that direction
        // Player Sprite is default looking to the West, so flip_x needs to be FALSE
        player_sprite.flip_x = false;
    } else if *new_direction == Directions::East {
        next_player_translation.x += MOVEMENT;
        // Also set player Sprite to that direction
        // Player Sprite is default looking to the West, so flip_x needs to be TRUE
        player_sprite.flip_x = true;
    } else if *new_direction == Directions::North {
        next_player_translation.y += MOVEMENT;
    } else if *new_direction == Directions::South {
        next_player_translation.y -= MOVEMENT;
    }

    // Check if the next player translation causes a collision
    collide(
        *collider_translation,
        *collider_size,
        next_player_translation,
        player_sprite.size,
    )
}
