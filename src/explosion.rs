use bevy::prelude::*;

use crate::{
    graphics::{AnimationTimer, SceneAssets},
    player::{despawn_player, Player},
    state::GameState,
};

#[derive(Component)]
struct Explosion;

#[derive(Event)]
pub struct ExplosionEndedEvent;

pub struct ExplosionPlugin;

impl Plugin for ExplosionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExplosionEndedEvent>()
            .add_systems(
                OnEnter(GameState::GameOver),
                (spawn_explosion, despawn_player).chain(),
            )
            .add_systems(
                Update,
                animate_explosion.run_if(in_state(GameState::GameOver)),
            );
    }
}

fn spawn_explosion(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    query: Query<&Transform, With<Player>>,
) {
    let player_transform = query.get_single().unwrap();
    commands.spawn((
        SpriteSheetBundle {
            texture: scene_assets.explosion.image.clone(),
            atlas: TextureAtlas {
                layout: unsafe {
                    scene_assets
                        .explosion
                        .texture_layout
                        .clone()
                        .unwrap_unchecked()
                },
                index: unsafe {
                    scene_assets
                        .explosion
                        .animation_indices
                        .clone()
                        .unwrap_unchecked()
                        .first
                },
            },
            transform: player_transform.clone(),
            ..default()
        },
        unsafe {
            scene_assets
                .explosion
                .animation_indices
                .clone()
                .unwrap_unchecked()
        },
        unsafe {
            scene_assets
                .explosion
                .animation_timer
                .clone()
                .unwrap_unchecked()
        },
        Explosion,
    ));
    commands.spawn(AudioBundle {
        source: scene_assets.explosion_audio.clone(),
        ..default()
    });
}

fn animate_explosion(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut AnimationTimer, &mut TextureAtlas), With<Explosion>>,
    mut explosion_event_writer: EventWriter<ExplosionEndedEvent>,
) {
    let Ok((entity, mut timer, mut atlas)) = query.get_single_mut() else {
        return;
    };
    timer.tick(time.delta());
    if timer.just_finished() {
        if atlas.index < 48 {
            atlas.index += 1;
        } else {
            commands.entity(entity).despawn_recursive();
            explosion_event_writer.send(ExplosionEndedEvent);
        }
    }
}
