use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct Score {
    pub p1: u32,
    pub p2: u32,
}