use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::distributions::{Distribution, Uniform};

use crate::{
    /*camera::Background, */ graphics::SceneAssets, player::Player, schedule::InGameSet,
    screen_bound_collision_detection::handle_screen_bound_collisions, state::GameState,
    ui::GameData,
};

pub const FIREBALL_SCALE: f32 = 0.049;
const FIREBALL_SPAWN_TIME: f64 = 10.0;
pub const INITIAL_FIREBALL_SPEED: f32 = 40.0;
const SPEED_MULTIPLIER: f32 = 1.75;
const FIREBALL_RADIUS: f32 = 512.0;

#[derive(Component)]
pub struct Fireball;

pub struct FireballPlugin;

#[derive(Resource, Default)]
pub struct FireballSpeed {
    speed: f32,
}

impl Plugin for FireballPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FireballSpeed>()
            .insert_resource(Time::<Fixed>::from_seconds(FIREBALL_SPAWN_TIME))
            .add_systems(
                FixedUpdate,
                spawn_fireball.run_if(in_state(GameState::InGame)),
            )
            .add_systems(
                Update,
                (/*rotate_fireball, */handle_screen_bound_collisions::<Fireball>)
                    .in_set(InGameSet::EntityUpdates),
            )
            .add_systems(FixedPostUpdate, increase_speed)
            .add_systems(OnExit(GameState::GameOver), despawn_fireballs);
    }
}

pub fn spawn_fireball(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_position_query: Query<&Transform, With<Player>>,
    scene_assets: Res<SceneAssets>,
    mut fireball_speed: ResMut<FireballSpeed>,
    mut game_data: ResMut<GameData>,
    // mut background_query: Query<Entity, With<Background>>,
) {
    // Remember to fire an event whenever we spawn a fireball in order to update the counter
    // Fireball speed should be incremented or multiplied every 10 balls spawned, think of creating anouther system or add additional checks here
    let Ok(player_pos) = player_position_query.get_single() else {
        return;
    };
    let window = window_query.get_single().unwrap();
    let height = window.height() / 2.0;
    let width = window.width() / 2.0;
    let between_width = Uniform::from(-width + 100.0..width - 100.0);
    let between_height = Uniform::from(-height + 100.0..height - 100.0);
    let mut rng = rand::thread_rng();

    let fireball_translation = Vec3::new(
        between_width.sample(&mut rng),
        between_height.sample(&mut rng),
        1.0,
    );

    (*fireball_speed).speed = game_data.current_fireballs_speed;

    // let background = background_query.get_single_mut().unwrap();

    let vel = (fireball_translation.xy() - player_pos.translation.xy()).normalize()
        * (*fireball_speed).speed;
    /*let fireball_id = */
    commands
        .spawn((
            SpriteBundle {
                texture: scene_assets.fireball.image.clone(),
                transform: Transform {
                    translation: fireball_translation,
                    // rotation: Quat::from_rotation_z(vel.angle_between(Vec2::new(-1.0, -1.0))),
                    // rotation: Quat::from_axis_angle(Vec3::Z, 1.57),
                    scale: Vec2::new(FIREBALL_SCALE, 0.0).xxy(),
                    ..default()
                },
                ..default()
            },
            Fireball,
            RigidBody::Dynamic,
        ))
        .insert(Velocity {
            linvel: vel,
            angvel: 0.0,
        })
        .insert(Damping {
            linear_damping: 0.0,
            angular_damping: 0.0,
        })
        .insert(Friction::coefficient(0.0))
        .insert(Restitution::coefficient(1.0))
        .insert(GravityScale(0.0))
        .insert(Collider::ball(FIREBALL_RADIUS))
        .insert(ActiveCollisionTypes::default() | ActiveCollisionTypes::KINEMATIC_KINEMATIC)
        .insert(ActiveEvents::COLLISION_EVENTS | ActiveEvents::CONTACT_FORCE_EVENTS);
    // .id();

    // commands.entity(background).push_children(&[fireball_id]);

    game_data.n_balls += 1;
}

// pub fn rotate_fireball(mut query: Query<(&Velocity, &mut Transform), With<Fireball>>) {
//     for (vel, mut transform) in &mut query {
//         transform.rotation = Quat::from_rotation_z(vel.linvel.angle_between(Vec2::new(0.0, 1.0)));
//     }
// }

pub fn despawn_fireballs(mut commands: Commands, fireball_query: Query<Entity, With<Fireball>>) {
    for entity in fireball_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn increase_speed(
    mut query: Query<&mut Velocity, With<Fireball>>,
    mut game_data: ResMut<GameData>,
) {
    if game_data.n_balls % 10 == 0 {
        game_data.current_fireballs_speed *= SPEED_MULTIPLIER;
        query
            .par_iter_mut()
            .for_each(|mut ball_velocity| ball_velocity.linvel *= SPEED_MULTIPLIER);
    }
}
