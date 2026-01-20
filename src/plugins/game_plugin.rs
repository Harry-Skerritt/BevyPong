use bevy::prelude::*;
use bevy::prelude::KeyCode::Space;
use bevy::ui::AlignItems::Default;
use crate::components::paddle::{Paddle, PaddleConstants};
use crate::components::player::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players);
    }
}


fn spawn_players(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Player { id: 1 },
        Paddle { width: PaddleConstants::WIDTH, height: PaddleConstants::HEIGHT },
        Movement { direction: None },
        Sprite {
            image: asset_server.load("sprites/paddleRed.png"),
            ..default()
        },
        Transform {
            translation: Vec3::new(PaddleConstants::X_OFFSET, 0.0, 0.0),
            rotation: Quat::from_rotation_z(90_f32.to_radians()),
            ..default()
        },
        GlobalTransform::default(),
    ));

    commands.spawn((
        Player { id: 2 },
        Paddle { width: PaddleConstants::WIDTH, height: PaddleConstants::HEIGHT },
        Movement { direction: None },
        Sprite {
            image: asset_server.load("sprites/paddleBlue.png"),
            ..default()
        },
        Transform {
            translation: Vec3::new(-PaddleConstants::X_OFFSET, 0.0, 0.0),
            rotation: Quat::from_rotation_z(90_f32.to_radians()),
            ..default()
        },
        GlobalTransform::default(),
    ));
}