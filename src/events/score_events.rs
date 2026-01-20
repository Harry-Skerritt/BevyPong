use bevy::prelude::*;

#[derive(Message)]
pub enum ScoreEvent {
    Player1,
    Player2,
}
