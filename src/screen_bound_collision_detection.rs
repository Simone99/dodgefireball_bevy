use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::dynamics::Velocity;

#[derive(Event)]
pub struct ScreenCollisionEvent {
    pub entity: Entity,
}

pub struct ScreenCollisionDetectionPlugin;

impl Plugin for ScreenCollisionDetectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ScreenCollisionEvent>();
    }
}

pub fn handle_screen_bound_collisions<T: Component>(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut component_query: Query<(Entity, &Transform, &mut Velocity), With<T>>,
    mut collision_event_writer: EventWriter<ScreenCollisionEvent>,
) {
    let window = window_query.get_single().unwrap();
    let height = window.height() / 2.0;
    let width = window.width() / 2.0;
    for (entity, transform, mut velocity) in &mut component_query {
        let point_east = (
            transform.translation.x + 2.0 * transform.scale.x + 2.0 * transform.scale.x,
            transform.translation.y,
        );
        let point_south = (
            transform.translation.x,
            transform.translation.y - 2.0 * transform.scale.y - 2.0 * transform.scale.y,
        );
        let point_west = (
            transform.translation.x - 2.0 * transform.scale.x - 2.0 * transform.scale.x,
            transform.translation.y,
        );
        let point_north = (
            transform.translation.x,
            transform.translation.y + 2.0 * transform.scale.y + 2.0 * transform.scale.y,
        );
        let mut send_event = false;
        if point_east.0 >= width || point_west.0 <= -width {
            (*velocity).linvel.x *= -1.0;
            send_event = true;
        }
        if point_south.1 >= height || point_north.1 < -height {
            (*velocity).linvel.y *= -1.0;
            send_event = true;
        }
        if send_event {
            collision_event_writer.send(ScreenCollisionEvent { entity });
        }
    }
}
