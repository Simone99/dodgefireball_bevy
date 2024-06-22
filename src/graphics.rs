// Check https://github.com/bevyengine/bevy/tree/latest/examples#examples

//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::prelude::*;

use crate::player::PLAYER_PIXELS;

#[derive(Component, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct AnimationTimer(Timer);

pub struct AnimatedEntity;
pub struct StaticEntity;

#[derive(Clone)]
pub struct EntityAssets<State = StaticEntity> {
    state: std::marker::PhantomData<State>,
    pub image: Handle<Image>,
    pub texture_layout: Option<Handle<TextureAtlasLayout>>,
    pub animation_indices: Option<AnimationIndices>,
    pub animation_timer: Option<AnimationTimer>,
}

#[derive(Resource, Default)]
pub struct SceneAssets {
    pub background: EntityAssets<StaticEntity>,
    pub player: EntityAssets<AnimatedEntity>,
    pub fireball: EntityAssets<StaticEntity>,
    pub explosion: EntityAssets<AnimatedEntity>,
    pub explosion_audio: Handle<AudioSource>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(PreStartup, load_assets);
    }
}

impl EntityAssets<StaticEntity> {
    pub fn new(image: Handle<Image>) -> Self {
        Self {
            state: std::marker::PhantomData::<StaticEntity>,
            image,
            texture_layout: None,
            animation_indices: None,
            animation_timer: None,
        }
    }
}

impl Default for EntityAssets<StaticEntity> {
    fn default() -> Self {
        Self {
            state: std::marker::PhantomData::<StaticEntity>,
            image: Default::default(),
            texture_layout: Default::default(),
            animation_indices: Default::default(),
            animation_timer: Default::default(),
        }
    }
}

impl Default for EntityAssets<AnimatedEntity> {
    fn default() -> Self {
        Self {
            state: std::marker::PhantomData::<AnimatedEntity>,
            image: Default::default(),
            texture_layout: Default::default(),
            animation_indices: Default::default(),
            animation_timer: Default::default(),
        }
    }
}

impl EntityAssets<AnimatedEntity> {
    pub fn new(
        image: Handle<Image>,
        texture_layout: Handle<TextureAtlasLayout>,
        animation_indices: AnimationIndices,
        animation_timer: AnimationTimer,
    ) -> Self {
        Self {
            state: std::marker::PhantomData::<AnimatedEntity>,
            image,
            texture_layout: Some(texture_layout),
            animation_indices: Some(animation_indices),
            animation_timer: Some(animation_timer),
        }
    }
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut scene_assets: ResMut<SceneAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let background = EntityAssets::<StaticEntity>::new(asset_server.load("background_1.png"));
    let explosion = EntityAssets::<AnimatedEntity>::new(
        asset_server.load("explosion_sequece.png"),
        texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            Vec2::new(240.0, 240.0),
            8,
            6,
            None,
            None,
        )),
        AnimationIndices {
            first: 0,
            last: 6 * 8,
        },
        AnimationTimer(Timer::from_seconds(0.04, TimerMode::Repeating)),
    );
    let fireball = EntityAssets::<StaticEntity>::new(asset_server.load("fireball.png"));
    let player = EntityAssets::<AnimatedEntity>::new(
        asset_server.load("player.png"),
        texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            Vec2::new(PLAYER_PIXELS, PLAYER_PIXELS),
            4,
            4,
            None,
            None,
        )),
        AnimationIndices {
            first: 0,
            last: 4 * 4,
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    );
    let explosion_audio = asset_server.load("explosion.ogg");
    *scene_assets = SceneAssets {
        background,
        player,
        fireball,
        explosion,
        explosion_audio,
    };
}
