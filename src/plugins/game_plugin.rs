use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::ball::{Ball, BallConstants, Velocity};
use crate::components::collider::Collider;
use crate::components::paddle::{Paddle, PaddleConstants};
use crate::components::player::*;
use crate::events::score_events::ScoreEvent;
use crate::systems::score::*;
use crate::resources::Score;
use crate::resources::score::WinTimer;
use crate::states::game_state::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        // states
        app.init_state::<GameState>();

        // Messages (Events)
        app
            .add_message::<ScoreEvent>();

        // Resources
        app.insert_resource(
            Score {
                p1: 0,
                p2: 0,
                win_score: 5,
            });

        // Startup Systems
        app
            .add_systems(Startup, (
                spawn_players,
                spawn_middle_line,
                spawn_ball
            ));

        // Update Systems
        app
            .add_systems(Update, update_score)
            .add_systems(Update, tick_win_timer.run_if(in_state(GameState::WinnerScreen)));

    }
}


fn spawn_players(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let p1_handle = asset_server.load("sprites/paddleBlue.png");
    let p2_handle = asset_server.load("sprites/paddleRed.png");


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
            translation: Vec3::new(PaddleConstants::X_OFFSET, 0.0, 1.0),
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
            translation: Vec3::new(-PaddleConstants::X_OFFSET, 0.0, 1.0),
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
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single().unwrap();
    let window_height = window.resolution.height();

    let dash_width = 4.0;
    let dash_height = 20.0;
    let dash_spacing = 20.0;
    let num_dashes = (window_height / (dash_height + dash_spacing)) as i32;

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

fn spawn_ball (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let ball_handle = asset_server.load("sprites/ball.png");

    commands.spawn((
        Ball,
        Velocity(BallConstants::START_VELOCITY),
        Sprite {
            image: ball_handle,
            custom_size: Some(Vec2::splat(BallConstants::RADIUS * 2.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        GlobalTransform::default(),
        Collider {
            size: Vec2::splat(BallConstants::RADIUS * 2.0),
        },
        ));
}



fn tick_win_timer(
    time: Res<Time>,
    mut win_resource: ResMut<WinTimer>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
    mut ball_query: Query<&mut Transform, With<Ball>>,
) {
    if win_resource.timer.tick(time.delta()).just_finished() {
        score.p1 = 0;
        score.p2 = 0;

        if let Ok(mut transform) = ball_query.single_mut() {
            transform.translation = Vec3::ZERO;
        }

        next_state.set(GameState::Playing);
    }
}