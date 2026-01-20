mod systems;

use std::cmp::PartialEq;
use bevy::prelude::*;
use bevy::window::SystemCursorIcon::Move;
use crate::systems::events::{MovementDirection, PlayerOneMovement, PlayerTwoMovement};
use crate::systems::input::keyboard_input_handler;

#[derive(Component)]
struct MainCamera;

fn main() {

    let mut app = App::new();

    app.add_plugins(
       DefaultPlugins.set(WindowPlugin {
           primary_window: Some(Window {
               title: "Bevy Pong!".to_string(),
               resolution: (1280, 720).into(),
               resizable: false,
               ..default()
           }),
           ..default()
       })
    );

    // Messages (Events)
    app
        .add_message::<PlayerOneMovement>()
        .add_message::<PlayerTwoMovement>();

    // Systems
    app
        .add_systems(Startup, camera_setup)
        .add_systems(Startup, sprite_setup)
        .add_systems(Update, player_movement_system)
        .add_systems(Update, keyboard_input_handler);


    app.run();
}

#[derive(Component)]
enum Direction {
    Left,
    Right,
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(( Camera2d, MainCamera ));
}

fn sprite_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("sprites/sprite.png")),
        Transform::from_xyz(0., 0., 0.),
        Direction::Right,
    ));
}


fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut logo, mut transform) in &mut sprite_position {
        match *logo {
            Direction::Right => transform.translation.x += 150. * time.delta_secs(),
            Direction::Left => transform.translation.x -= 150. * time.delta_secs(),
        }

        if transform.translation.x > 200. {
            *logo = Direction::Left;
        } else if transform.translation.x < -200. {
            *logo = Direction::Right;
        }
    }
}


fn player_movement_system(
    mut reader_one: MessageReader<PlayerOneMovement>,
    mut reader_two: MessageReader<PlayerTwoMovement>,
) {
    for event_one in reader_one.read() {
        if (event_one.direction == MovementDirection::Up && event_one.pressed) {
            info!("Player One Moving Up!");
        }

        if (event_one.direction == MovementDirection::Down && event_one.pressed) {
            info!("Player One Moving Down!");
        }
    }

    for event_two in reader_two.read() {
        if (event_two.direction == MovementDirection::Up && event_two.pressed) {
            info!("Player Two Moving Up!");
        }

        if (event_two.direction == MovementDirection::Down && event_two.pressed) {
            info!("Player Two Moving Down!");
        }
    }
}
