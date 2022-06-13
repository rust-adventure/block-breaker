use bevy::prelude::*;

use crate::BallHit;

pub fn ball_hit_particles(
    mut commands: Commands,
    mut particles: Query<
        (Entity, &mut Sprite),
        With<BallHit>,
    >,
) {
    for (entity, mut sprite) in particles.iter_mut() {
        if sprite.custom_size.unwrap().x > 100.0 {
            commands.entity(entity).despawn_recursive()
        } else {
            sprite.custom_size = Some(
                sprite.custom_size.unwrap()
                    + Vec2::new(5.0, 5.0),
            );
        }
    }
}
