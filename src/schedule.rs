use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
use bevy_rapier2d::plugin::{NoUserData, PhysicsSet, RapierPhysicsPlugin};

use crate::state::GameState;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum InGameSet {
    UserInput,
    EntityUpdates,
    CollisionDetection,
    DespwanEntities,
}

#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PhysicsSchedule;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                InGameSet::DespwanEntities,
                InGameSet::UserInput,
                InGameSet::EntityUpdates,
                InGameSet::CollisionDetection,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            apply_deferred
                .after(InGameSet::DespwanEntities)
                .before(InGameSet::UserInput),
        )
        .add_plugins(
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0)
                .with_default_system_setup(false),
        )
        .add_systems(
            PhysicsSchedule,
            (
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::SyncBackend)
                    .in_set(PhysicsSet::SyncBackend),
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::StepSimulation)
                    .in_set(PhysicsSet::StepSimulation),
                RapierPhysicsPlugin::<NoUserData>::get_systems(PhysicsSet::Writeback)
                    .in_set(PhysicsSet::Writeback),
            ),
        )
        .init_schedule(PhysicsSchedule)
        .edit_schedule(PhysicsSchedule, |schedule| {
            schedule.configure_sets(
                (
                    PhysicsSet::SyncBackend,
                    PhysicsSet::StepSimulation,
                    PhysicsSet::Writeback,
                )
                    .chain()
                    .run_if(in_state(GameState::InGame)),
            );
        })
        .add_systems(PreUpdate, run_physics_schedule);
    }
}

pub fn run_physics_schedule(world: &mut World) {
    world.run_schedule(PhysicsSchedule);
}
