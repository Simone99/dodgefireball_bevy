mod camera;
mod explosion;
mod fireball;
mod graphics;
mod player;
mod scene;
mod schedule;
mod screen_bound_collision_detection;
mod state;
mod ui;

use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{PresentMode, WindowTheme},
};
use camera::{CameraPlugin, BACKGROUND_SCALE};
use explosion::ExplosionPlugin;
use fireball::FireballPlugin;
use graphics::AssetLoaderPlugin;
use player::PlayerPlugin;
use schedule::SchedulePlugin;
use screen_bound_collision_detection::ScreenCollisionDetectionPlugin;
use state::StatePlugin;
use ui::UiPlugin;

#[bevy_main]
fn main() {
    run_game();
}

pub fn run_game() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 0.75,
        })
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "DodgeFireBall".into(),
                    name: Some("dodgefireball.app".into()),
                    resolution: (448. * BACKGROUND_SCALE, 298. * BACKGROUND_SCALE).into(),
                    present_mode: PresentMode::AutoVsync,
                    // Tells wasm not to override default event handling, like F5, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    window_theme: Some(WindowTheme::Dark),
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    // This will spawn an invisible window
                    // The window will be made visible in the make_visible() system after 3 frames.
                    // This is useful when you want to avoid the white window that shows up before the GPU is ready to render the app.
                    visible: true,
                    ..default()
                }),
                ..default()
            }),
            LogDiagnosticsPlugin::default(),
            FrameTimeDiagnosticsPlugin,
        ))
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(FireballPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(UiPlugin)
        .add_plugins(ScreenCollisionDetectionPlugin)
        .add_plugins(StatePlugin)
        .add_plugins(ExplosionPlugin)
        .add_plugins(SchedulePlugin)
        .run();
}
