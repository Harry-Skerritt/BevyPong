mod systems;
mod components;
mod plugins;
mod events;
mod resources;
mod states;

use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use plugins::{ PlayerPlugin, GamePlugin, BallPlugin, UiPlugin };

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


    // Plugins
    app
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(GamePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(BallPlugin)
        .add_plugins(UiPlugin);


    // Systems
    app.add_systems(Startup, camera_setup);


    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(( Camera2d, MainCamera ));
}
