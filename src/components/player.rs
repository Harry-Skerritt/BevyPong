use bevy::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MovementDirection {
    Up,
    Down,
}

#[derive(Component)]
pub struct Player {
    pub id: u8,
}

#[derive(Component)]
pub struct Movement {
    pub direction: Option<MovementDirection>,
}

