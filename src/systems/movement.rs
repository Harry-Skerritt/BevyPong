use bevy::prelude::*;
use crate::components::player::*;
use crate::components::paddle::*;
pub fn paddle_movement_system(
    time: Res<Time>,
    mut query: Query<(&Movement, &mut Transform), With<Paddle>>,
) {
    let speed = PaddleConstants::SPEED;

    for (movement, mut transform) in query.iter_mut() {
        if let Some(dir) = movement.direction {
            match dir {
                MovementDirection::Up => transform.translation.y += speed * time.delta_secs(),
                MovementDirection::Down => transform.translation.y -= speed * time.delta_secs(),
            }
        }
    }
}