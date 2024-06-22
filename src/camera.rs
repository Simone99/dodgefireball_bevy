use bevy::{prelude::*, window::WindowResized};

use crate::graphics::SceneAssets;

pub const BACKGROUND_SCALE: f32 = 3.1;
const CAMERA_DISTANCE: f32 = 80.0;

#[derive(Component)]
pub struct Background;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_camera, spawn_background))
            .add_systems(Update, on_resize_system);
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, CAMERA_DISTANCE).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

pub fn spawn_background(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        SpriteBundle {
            texture: scene_assets.background.image.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -1.0),
                scale: Vec2::new(BACKGROUND_SCALE, 0.0).xxy(),
                ..default()
            },
            ..default()
        },
        Background,
    ));
}

fn on_resize_system(
    mut q: Query<&mut Transform, With<Background>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    let mut background_transform = q.single_mut();
    for e in resize_reader.read() {
        // When resolution is being changed
        background_transform.scale = Vec3::new(e.width / 445.0, e.height / 295.0, 0.0);
    }
}
