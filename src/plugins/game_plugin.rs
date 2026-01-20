use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::collider::Collider;
use crate::components::paddle::{Paddle, PaddleConstants};
use crate::components::player::*;
use crate::systems::collisions::*;


pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players)
            .add_systems(Startup, spawn_middle_line)
            .add_systems(Update, clamp_paddle_collisions);
    }
}


fn spawn_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let p1_handle = asset_server.load("sprites/paddleRed.png");
    let p2_handle = asset_server.load("sprites/paddleBlue.png");


    // Player 1
    commands.spawn((
        Player { id: 1 },
        Paddle,
        Movement { direction: None },
        Sprite {
            image: p1_handle,
            custom_size: Some(Vec2::new(PaddleConstants::WIDTH, PaddleConstants::HEIGHT)),
            ..default()
        },
        Transform {
            translation: Vec3::new(PaddleConstants::X_OFFSET, 0.0, 0.0),
            ..default()
        },
        GlobalTransform::default(),
        Collider {
            size: Vec2::new(PaddleConstants::WIDTH, PaddleConstants::HEIGHT),
        }
    ));

    // Player 2
    commands.spawn((
        Player { id: 2 },
        Paddle,
        Movement { direction: None },
        Sprite {
            image: p2_handle,
            custom_size: Some(Vec2::new(PaddleConstants::WIDTH, PaddleConstants::HEIGHT)),
            ..default()
        },
        Transform {
            translation: Vec3::new(-PaddleConstants::X_OFFSET, 0.0, 0.0),
            ..default()
        },
        GlobalTransform::default(),
        Collider {
            size: Vec2::new(PaddleConstants::WIDTH, PaddleConstants::HEIGHT),
        }
    ));
}


fn spawn_middle_line(
    mut commands: Commands,
    query_window: Query<&Window, With<PrimaryWindow>>,
) {
    let window = match query_window.single() {
        Ok(window) => window,
        Err(_) => return
    };
    
    let window_height = window.resolution.height();

    let dash_width = 4.0;
    let dash_height = 20.0;
    let dash_spacing = 20.0;
    let num_dashes = (window_height / (dash_height + dash_spacing)) as i32; // Todo: Get window height here

    for i in 0..num_dashes {
        let y = -(window_height / 2.0) + i as f32 * (dash_height + dash_spacing) + dash_height / 2.0;
        commands.spawn((
            Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(dash_width, dash_height)),
                    ..default()
            },
            Transform {
                translation: Vec3::new (-(dash_width / 2.0), y, 0.0),
                ..default()
            }
        ));
    }
}
