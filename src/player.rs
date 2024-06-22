use bevy::prelude::*;
#[cfg(target_os = "android")]
use bevy::window::PrimaryWindow;
use bevy_rapier2d::{pipeline::CollisionEvent, prelude::*};

use crate::{
    graphics::{AnimationTimer, SceneAssets},
    schedule::InGameSet,
    screen_bound_collision_detection::{handle_screen_bound_collisions, ScreenCollisionEvent},
    state::GameState,
};

const PLAYER_SCALE: f32 = 1.0;
const INITIAL_VELOCITY: f32 = 100.0;
const INITIAL_DIRECTION: PlayerDirection = PlayerDirection::Down;
pub const PLAYER_PIXELS: f32 = 64.0;

#[derive(Component)]
pub struct Player;

#[derive(Resource)]
pub struct PlayerController {
    pub enabled: bool,
}

pub struct PlayerPlugin;

#[derive(Component, Default, Clone)]
pub enum PlayerDirection {
    Up,
    #[default]
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerController>()
            .add_systems(Startup, spawn_player)
            .add_systems(OnExit(GameState::GameOver), spawn_player)
            .add_systems(Update, (player_controller).in_set(InGameSet::UserInput))
            .add_systems(
                Update,
                (
                    check_player_death,
                    animate_player,
                    handle_screen_bound_collisions::<Player>,
                    apply_screen_collision,
                )
                    .chain()
                    .in_set(InGameSet::EntityUpdates),
            );
    }
}

impl Default for PlayerController {
    fn default() -> Self {
        Self { enabled: true }
    }
}

impl PlayerDirection {
    fn to_direction2d(&self) -> Direction2d {
        match self {
            PlayerDirection::Up => Direction2d::new_unchecked(Vec2::new(0.0, 1.0)),
            PlayerDirection::Down => Direction2d::new_unchecked(Vec2::new(0.0, -1.0)),
            PlayerDirection::Left => Direction2d::new_unchecked(Vec2::new(-1.0, 0.0)),
            PlayerDirection::Right => Direction2d::new_unchecked(Vec2::new(1.0, 0.0)),
            PlayerDirection::UpLeft => Direction2d::new_unchecked(Vec2::new(-1.0, 1.0).normalize()),
            PlayerDirection::UpRight => Direction2d::new_unchecked(Vec2::new(1.0, 1.0).normalize()),
            PlayerDirection::DownLeft => {
                Direction2d::new_unchecked(Vec2::new(-1.0, -1.0).normalize())
            }
            PlayerDirection::DownRight => {
                Direction2d::new_unchecked(Vec2::new(1.0, -1.0).normalize())
            }
        }
    }
    fn from_direction2d(vel: Vec2) -> PlayerDirection {
        let direction = Direction2d::new_unchecked(vel.normalize());
        if direction == Direction2d::new_unchecked(Vec2::new(0.0, 1.0)) {
            PlayerDirection::Up
        } else if direction == Direction2d::new_unchecked(Vec2::new(0.0, -1.0)) {
            PlayerDirection::Down
        } else if direction == Direction2d::new_unchecked(Vec2::new(-1.0, 0.0)) {
            PlayerDirection::Left
        } else if direction == Direction2d::new_unchecked(Vec2::new(1.0, 0.0)) {
            PlayerDirection::Right
        } else if direction == Direction2d::new_unchecked(Vec2::new(-1.0, 1.0).normalize()) {
            PlayerDirection::UpLeft
        } else if direction == Direction2d::new_unchecked(Vec2::new(1.0, 1.0).normalize()) {
            PlayerDirection::UpRight
        } else if direction == Direction2d::new_unchecked(Vec2::new(-1.0, -1.0).normalize()) {
            PlayerDirection::DownLeft
        } else if direction == Direction2d::new_unchecked(Vec2::new(1.0, -1.0).normalize()) {
            PlayerDirection::DownRight
        } else {
            PlayerDirection::Down
        }
    }
}

pub fn spawn_player(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands
        .spawn((
            SpriteSheetBundle {
                texture: scene_assets.player.image.clone(),
                atlas: TextureAtlas {
                    layout: unsafe {
                        scene_assets
                            .player
                            .texture_layout
                            .clone()
                            .unwrap_unchecked()
                    },
                    index: unsafe {
                        scene_assets
                            .player
                            .animation_indices
                            .clone()
                            .unwrap_unchecked()
                            .first
                    },
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    scale: Vec2::new(PLAYER_SCALE, 0.0).xxy(),
                    ..default()
                },
                ..default()
            },
            unsafe {
                scene_assets
                    .player
                    .animation_indices
                    .clone()
                    .unwrap_unchecked()
            },
            unsafe {
                scene_assets
                    .player
                    .animation_timer
                    .clone()
                    .unwrap_unchecked()
            },
            Player,
            RigidBody::Dynamic,
            INITIAL_DIRECTION,
        ))
        .insert(Velocity {
            linvel: INITIAL_DIRECTION
                .to_direction2d()
                .rotate(Vec2::new(INITIAL_VELOCITY, 0.0)),
            angvel: 0.0,
        })
        .insert(Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        })
        .insert(GravityScale(0.0))
        .insert(Collider::cuboid(PLAYER_PIXELS / 2.0, PLAYER_PIXELS / 2.0))
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC)
        .insert(ActiveEvents::COLLISION_EVENTS);
    // .insert(Friction::coefficient(0.0))
    // .insert(ColliderMassProperties::Density(2.0));
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlas, &PlayerDirection), With<Player>>,
) {
    let (mut timer, mut atlas, direction) = query.get_single_mut().unwrap();
    timer.tick(time.delta());
    if timer.just_finished() {
        match direction {
            PlayerDirection::Up => atlas.index = (atlas.index + 1) % 4 + 4 * 3,
            PlayerDirection::Down => atlas.index = (atlas.index + 1) % 4,
            PlayerDirection::Left | PlayerDirection::UpLeft | PlayerDirection::DownLeft => {
                atlas.index = (atlas.index + 1) % 4 + 4
            }
            PlayerDirection::Right | PlayerDirection::UpRight | PlayerDirection::DownRight => {
                atlas.index = (atlas.index + 1) % 4 + 4 * 2
            }
        }
    }
}

#[cfg(not(target_os = "android"))]
fn player_controller(
    mut query: Query<(&mut PlayerDirection, &mut Velocity), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_controller: ResMut<PlayerController>,
) {
    let Ok((mut player_direction, mut velocity)) = query.get_single_mut() else {
        return;
    };
    if player_controller.enabled {
        let mut new_direction = None;
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                new_direction = Some(PlayerDirection::DownLeft);
            } else if keyboard_input.pressed(KeyCode::ArrowUp) {
                new_direction = Some(PlayerDirection::UpLeft);
            } else {
                new_direction = Some(PlayerDirection::Left);
            }
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            if keyboard_input.pressed(KeyCode::ArrowDown) {
                new_direction = Some(PlayerDirection::DownRight);
            } else if keyboard_input.pressed(KeyCode::ArrowUp) {
                new_direction = Some(PlayerDirection::UpRight);
            } else {
                new_direction = Some(PlayerDirection::Right);
            }
        } else if keyboard_input.pressed(KeyCode::ArrowUp) {
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                new_direction = Some(PlayerDirection::UpLeft);
            } else if keyboard_input.pressed(KeyCode::ArrowRight) {
                new_direction = Some(PlayerDirection::UpRight);
            } else {
                new_direction = Some(PlayerDirection::Up);
            }
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                new_direction = Some(PlayerDirection::DownLeft);
            } else if keyboard_input.pressed(KeyCode::ArrowRight) {
                new_direction = Some(PlayerDirection::DownRight);
            } else {
                new_direction = Some(PlayerDirection::Down);
            }
        }
        if let Some(direction) = new_direction {
            *player_direction = direction.clone();
            *velocity = Velocity {
                linvel: direction
                    .to_direction2d()
                    .rotate(Vec2::new(INITIAL_VELOCITY, 0.0)),
                angvel: 0.0,
            };
        }
    } else {
        if !keyboard_input.pressed(KeyCode::ArrowLeft)
            && !keyboard_input.pressed(KeyCode::ArrowRight)
            && !keyboard_input.pressed(KeyCode::ArrowUp)
            && !keyboard_input.pressed(KeyCode::ArrowDown)
        {
            player_controller.enabled = true;
        }
    }
}

#[cfg(target_os = "android")]
fn player_controller(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut PlayerDirection, &Transform, &mut Velocity), With<Player>>,
    mut touch_events: EventReader<TouchInput>,
) {
    let Ok((mut player_direction, player_transform, mut velocity)) = query.get_single_mut() else {
        return;
    };
    let window = window_query.get_single().unwrap();
    let height = window.height() / 2.0;
    let width = window.width() / 2.0;
    let screen_center = Vec2::new(width, height);
    let zero_vec = Vec2::new(-1.0, 0.0);
    for event in touch_events.read() {
        let new_direction_vec = screen_center - event.position;
        // info!(
        //     "{}",
        //     new_direction_vec
        //         .angle_between(zero_vec)
        //         .to_degrees()
        // );
        let direction = match new_direction_vec.angle_between(zero_vec).to_degrees() {
            -22.5..=22.5 => PlayerDirection::Right,
            22.5..=67.5 => PlayerDirection::UpRight,
            67.5..=112.5 => PlayerDirection::Up,
            112.5..=157.5 => PlayerDirection::UpLeft,
            157.5..=180.0 | -180.0..=-157.5 => PlayerDirection::Left,
            -157.5..=-112.5 => PlayerDirection::DownLeft,
            -112.5..=-67.5 => PlayerDirection::Down,
            -67.5..=-22.5 => PlayerDirection::DownRight,
            _ => PlayerDirection::Down,
        };
        *player_direction = direction.clone();
        *velocity = Velocity {
            linvel: direction
                .to_direction2d()
                .rotate(Vec2::new(INITIAL_VELOCITY, 0.0)),
            angvel: 0.0,
        };
    }
}

fn apply_screen_collision(
    mut collision_event_reader: EventReader<ScreenCollisionEvent>,
    mut player_controller: ResMut<PlayerController>,
    mut query: Query<(Entity, &mut PlayerDirection, &Velocity), With<Player>>,
) {
    let (player_entity, mut player_direction, player_velocity) = query.get_single_mut().unwrap();
    for &ScreenCollisionEvent { entity } in collision_event_reader.read() {
        if entity == player_entity {
            player_controller.enabled = false;
            *player_direction = PlayerDirection::from_direction2d(player_velocity.linvel);
        }
    }
}

fn check_player_death(
    mut collision_event_reader: EventReader<CollisionEvent>,
    query: Query<Entity, With<Player>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Ok(player_entity) = query.get_single() else {
        return;
    };
    for collision in collision_event_reader.read() {
        match collision {
            CollisionEvent::Started(e1, e2, _) => {
                if e1 == &player_entity || e2 == &player_entity {
                    next_state.set(GameState::GameOver);
                }
            }
            CollisionEvent::Stopped(_e1, _e2, _) => {}
        }
    }
}

pub fn despawn_player(mut commands: Commands, query: Query<Entity, With<Player>>) {
    let player_entity = query.get_single().unwrap();
    commands.entity(player_entity).despawn_recursive();
}
