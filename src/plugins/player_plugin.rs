use bevy::prelude::*;
use crate::systems::input::keyboard_input_handler;
use crate::systems::movement::paddle_movement_system;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, keyboard_input_handler)
            .add_systems(Update, paddle_movement_system);
    }
}