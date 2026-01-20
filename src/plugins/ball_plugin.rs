use bevy::prelude::*;
use crate::states::game_state::GameState;
use crate::systems::collisions::{ball_window_collisions, ball_paddle_collisions };
use crate::systems::movement::ball_movement_system;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                ball_movement_system,
                ball_window_collisions,
                ball_paddle_collisions
            ).run_if(in_state(GameState::Playing)));
    }
}