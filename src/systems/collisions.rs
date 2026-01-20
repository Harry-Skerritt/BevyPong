use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::paddle::{ Paddle, PaddleConstants };
use crate::components::ball::{ Ball, Velocity };
use crate::components::collider::Collider;


// --- PADDLE ---
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

// --- BALL ---
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

pub fn ball_paddle_collisions(
    mut ball_query: Query<(&mut Transform, &mut Velocity, &Collider), (With<Ball>, Without<Paddle>)>,
    paddle_query: Query<(&Transform, &Collider), With<Paddle>>,
) {
    for (ball_transform, mut velocity, ball_collider) in &mut ball_query {
        for (paddle_transform, paddle_collider) in paddle_query.iter() {

            let collision_x = (ball_transform.translation.x - paddle_transform.translation.x).abs()
                < (ball_collider.size.x / 2.0 + paddle_collider.size.x / 2.0);

            let collision_y = (ball_transform.translation.y - paddle_transform.translation.y).abs()
                < (ball_collider.size.y / 2.0 + paddle_collider.size.y / 2.0);

            if collision_x && collision_y {
                velocity.x = -velocity.x;
                let diff = ball_transform.translation.y - paddle_transform.translation.y;
                velocity.y += diff * 2.0;
            }
        }
    }
}