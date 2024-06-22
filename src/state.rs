use bevy::prelude::*;

use crate::explosion::ExplosionEndedEvent;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

#[derive(Resource, Default)]
pub struct StateFlags {
    pub explosion_ended: bool,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StateFlags>()
            .insert_state(GameState::default())
            .add_systems(OnEnter(GameState::InGame), restart_time)
            .add_systems(
                Update,
                (check_explosion_ended, game_state_input_events).chain(),
            );
    }
}

pub fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state_flags: ResMut<StateFlags>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::InGame => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::InGame),
            GameState::GameOver => {
                if state_flags.explosion_ended {
                    next_state.set(GameState::InGame);
                    state_flags.explosion_ended = false;
                }
            }
        }
    }
}

fn restart_time(mut time: ResMut<Time<Virtual>>) {
    time.unpause()
}

fn check_explosion_ended(
    mut explosion_event_reader: EventReader<ExplosionEndedEvent>,
    mut state_flags: ResMut<StateFlags>,
) {
    for _ in explosion_event_reader.read() {
        state_flags.explosion_ended = true;
        break;
    }
}
