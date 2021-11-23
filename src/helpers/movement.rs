use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_ecs_tilemap::MapQuery;

#[allow(dead_code)]
pub struct Player;

// A simple camera system for moving and zooming the camera.
#[allow(dead_code)]
pub fn update(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    map_query: MapQuery,
) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        transform.translation += time.delta_seconds() * direction * 50.;

        let sprite_pos_z =
            map_query.get_zindex_for_pixel_pos(transform.translation.xy().extend(1.0), 0u16, 1u16);
        transform.translation.z = sprite_pos_z;
    }
}
