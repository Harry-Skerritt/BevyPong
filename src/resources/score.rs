use bevy::prelude::{Resource, Timer};

#[derive(Resource, Default)]
pub struct Score {
    pub p1: u32,
    pub p2: u32,
    pub win_score: u32,
}

#[derive(Resource)]
pub struct WinTimer {
    pub timer: Timer,
    pub winner: u32
}