use bevy::prelude::*;

use crate::{
    camera::{spawn_background, spawn_camera},
    fireball::spawn_fireball,
    graphics::{load_assets, SceneAssets},
    player::spawn_player,
};

pub struct SceneBuilderPlugin;

impl Plugin for SceneBuilderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>().add_systems(
            Startup,
            (
                load_assets,
                spawn_camera,
                spawn_background,
                spawn_player,
                spawn_fireball,
            )
                .chain(),
        );
    }
}
