use bevy::prelude::*;

pub struct BallConstants;
impl BallConstants {
    pub const RADIUS: f32 = 16.0;
    pub const START_VELOCITY: Vec2 = Vec2::new(300.0, 200.0);
}

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);