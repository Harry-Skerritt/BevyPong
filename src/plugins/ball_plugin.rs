use bevy::prelude::*;
use crate::systems::collisions::ball_window_collisions;
use crate::systems::movement::ball_movement_system;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, ball_movement_system)
            .add_systems(Update, ball_window_collisions);
    }
}