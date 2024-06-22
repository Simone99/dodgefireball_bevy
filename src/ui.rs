use crate::{fireball::INITIAL_FIREBALL_SPEED, state::GameState};
use bevy::{prelude::*, window::PrimaryWindow};
#[cfg(not(target_os = "android"))]
use bevy_pkv::PkvStore;

const FONT_SIZE: f32 = 60.0;

#[derive(Resource)]
pub struct GameData {
    pub record: u64,
    pub n_balls: u64,
    pub current_fireballs_speed: f32,
}

#[derive(Component)]
pub struct UiComponent;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameData>()
            .add_systems(Startup, spawn_ui)
            .add_systems(Update, update_ui)
            .add_systems(OnExit(GameState::GameOver), reset_score);
        #[cfg(not(target_os = "android"))]
        app.insert_resource(PkvStore::new("Simomaster1", "DodgeFireBall"))
            .add_systems(OnEnter(GameState::GameOver), store_new_record);
    }
}

impl Default for GameData {
    fn default() -> Self {
        Self {
            record: Default::default(),
            n_balls: Default::default(),
            current_fireballs_speed: INITIAL_FIREBALL_SPEED,
        }
    }
}

fn spawn_ui(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    #[cfg(not(target_os = "android"))] mut game_data: ResMut<GameData>,
    #[cfg(target_os = "android")] game_data: Res<GameData>,
    #[cfg(not(target_os = "android"))] mut pkv: ResMut<PkvStore>,
) {
    #[cfg(not(target_os = "android"))]
    if let Ok(record) = pkv.get::<u64>("best_score") {
        game_data.record = record;
    } else {
        pkv.set("best_score", &0u64)
            .expect("failed to store best score");
    }
    let window = window_query.get_single().unwrap();
    let height = window.height();
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                format!("{}\nHigh Score: {}", game_data.n_balls, game_data.record),
                TextStyle {
                    font_size: FONT_SIZE,
                    color: Color::WHITE,
                    ..default()
                },
            ) // You can still add text justifaction.
            .with_justify(JustifyText::Center),
            transform: Transform {
                translation: Vec3::new(0.0, height / 2.0 - FONT_SIZE, 2.0),
                ..default()
            },
            ..default()
        },
        UiComponent,
    ));
}

fn update_ui(
    mut query: Query<(&mut Text, &mut Transform), With<UiComponent>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_data: Res<GameData>,
) {
    let window = window_query.get_single().unwrap();
    let height = window.height();
    let (mut ui_text, mut ui_transform) = query.get_single_mut().unwrap();
    *ui_text = Text::from_section(
        format!("{}\nHigh Score: {}", game_data.n_balls, game_data.record),
        TextStyle {
            font_size: FONT_SIZE,
            color: Color::WHITE,
            ..default()
        },
    ) // You can still add text justifaction.
    .with_justify(JustifyText::Center);
    ui_transform.translation = Vec3::new(0.0, height / 2.0 - FONT_SIZE, 2.0);
}

fn reset_score(mut game_data: ResMut<GameData>) {
    game_data.n_balls = 0;
}

#[cfg(not(target_os = "android"))]
fn store_new_record(mut pkv: ResMut<PkvStore>, mut game_data: ResMut<GameData>) {
    if game_data.n_balls > game_data.record {
        game_data.record = game_data.n_balls;
        pkv.set("best_score", &game_data.n_balls)
            .expect("failed to store best score");
    }
}
