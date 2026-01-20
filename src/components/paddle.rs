use bevy::prelude::*;


pub struct PaddleConstants;

impl PaddleConstants {
    pub const WIDTH: f32 = 20.0;
    pub const HEIGHT: f32 = 100.0;
    pub const SPEED: f32 = 500.0;
    pub const X_OFFSET: f32 = 620.0;
}

#[derive(Component)]
pub struct Paddle;