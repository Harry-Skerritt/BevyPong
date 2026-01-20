use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::paddle::{Paddle, PaddleConstants};

pub fn clamp_paddle_collisions(
    query_window: Query<&Window, With<PrimaryWindow>>,
    mut query_paddles: Query<&mut Transform, With<Paddle>>,
) {
    let window = match query_window.single() {
        Ok(win) => win,
        Err(_) => return,
    };

    let half_height = window.resolution.height() / 2.0;

    for mut transform in &mut query_paddles {
        let half_paddle = PaddleConstants::HEIGHT / 2.0;

        let y = transform.translation.y;
        transform.translation.y =
            y.clamp(-half_height + half_paddle, half_height - half_paddle);
    }
}