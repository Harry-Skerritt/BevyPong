use bevy::prelude::*;
use crate::components::player::{Player, Movement, MovementDirection};

pub fn keyboard_input_handler (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &mut Movement)>,
) {

    for (player, mut movement) in query.iter_mut() {
        movement.direction = None;

        match player.id {
            1 => {
                if keyboard_input.pressed(KeyCode::ArrowUp) {
                    movement.direction = Some(MovementDirection::Up);
                } else if keyboard_input.pressed(KeyCode::ArrowDown) {
                    movement.direction = Some(MovementDirection::Down);
                }
            }
            2 => {
                if keyboard_input.pressed(KeyCode::KeyW) {
                    movement.direction = Some(MovementDirection::Up);
                } else if keyboard_input.pressed(KeyCode::KeyS) {
                    movement.direction = Some(MovementDirection::Down);
                }
            }
            _ => {}
        }
    }

}