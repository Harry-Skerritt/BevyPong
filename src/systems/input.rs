use bevy::prelude::*;
use crate::systems::events::*;

pub fn keyboard_input_handler (
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_one_wrt: MessageWriter<PlayerOneMovement>,
    mut player_two_wrt: MessageWriter<PlayerTwoMovement>,
) {

    let player_one_keys = [
        (KeyCode::ArrowUp, MovementDirection::Up),
        (KeyCode::ArrowDown, MovementDirection::Down),
    ];

    let player_two_keys = [
        (KeyCode::KeyW, MovementDirection::Up),
        (KeyCode::KeyS, MovementDirection::Down),
    ];


    for (key, dir) in &player_one_keys {
        if keyboard_input.just_pressed(*key) {
            player_one_wrt.write(PlayerOneMovement {
                direction: *dir,
                pressed: true,
            });
        }

        if keyboard_input.just_released(*key) {
            player_one_wrt.write(PlayerOneMovement {
                direction: *dir,
                pressed: false,
            });
        }
    }

    for (key, dir) in &player_two_keys {
        if keyboard_input.just_pressed(*key) {
            player_two_wrt.write(PlayerTwoMovement {
                direction: *dir,
                pressed: true,
            });
        }

        if keyboard_input.just_released(*key) {
            player_two_wrt.write(PlayerTwoMovement {
                direction: *dir,
                pressed: false,
            });
        }
    }
}