use bevy::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MovementDirection {
    Up,
    Down,
}

#[derive(Debug)]
pub struct PlayerMovement {
    pub direction: MovementDirection,
    pub pressed: bool,
}

#[derive(Message, Debug)]
pub struct PlayerOneMovement {
    pub direction: MovementDirection,
    pub pressed: bool,
}

#[derive(Message, Debug)]
pub struct PlayerTwoMovement {
    pub direction: MovementDirection,
    pub pressed: bool,
}

