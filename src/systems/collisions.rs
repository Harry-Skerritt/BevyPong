use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::paddle::{ Paddle, PaddleConstants };
use crate::components::ball::{ Ball, Velocity };
use crate::components::collider::Collider;

pub fn clamp_paddle_collisions(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query_paddles: Query<&mut Transform, With<Paddle>>,
) {
    let window = windows.single().unwrap();
    let half_height = window.resolution.height() / 2.0;

    for mut transform in &mut query_paddles {
        let half_paddle = PaddleConstants::HEIGHT / 2.0;

        let y = transform.translation.y;
        transform.translation.y =
            y.clamp(-half_height + half_paddle, half_height - half_paddle);
    }
}

pub fn ball_window_collisions(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<(&mut Transform, &mut Velocity, &Collider), With<Ball>>,
) {
    let window = windows.single().unwrap();
    let half_height = window.resolution.height() / 2.0;
    let half_width = window.resolution.width() / 2.0;

    for (mut transform, mut velocity, collider) in &mut query {
        let half_ball = collider.size / 2.0;

        // Top / Bottom
        if transform.translation.y + half_ball.y > half_height {
            transform.translation.y = half_height - half_ball.y;
            velocity.y = -velocity.y;
        } else if transform.translation.y - half_ball.y < -half_height {
            transform.translation.y = -half_height + half_ball.y;
            velocity.y = -velocity.y;
        }

        // Left / Right
        if transform.translation.x + half_ball.x > half_width {
            println!("Player 1 scores!");
            transform.translation = Vec3::ZERO;
        } else if transform.translation.x - half_ball.x < -half_width {
            println!("Player 2 scores!");
            transform.translation = Vec3::ZERO;
        }
    }


}