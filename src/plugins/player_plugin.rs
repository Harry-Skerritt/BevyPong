use bevy::prelude::*;
use crate::states::game_state::GameState;
use crate::systems::collisions::clamp_paddle_collisions;
use crate::systems::input::keyboard_input_handler;
use crate::systems::movement::paddle_movement_system;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                keyboard_input_handler,
                paddle_movement_system,
                clamp_paddle_collisions
            ).run_if(in_state(GameState::Playing)));
    }
}