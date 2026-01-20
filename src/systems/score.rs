use bevy::prelude::{MessageReader, ResMut};
use crate::events::score_events::ScoreEvent;
use crate::resources::Score;

pub fn update_score(
    mut reader: MessageReader<ScoreEvent>,
    mut score: ResMut<Score>,
) {
    for event in reader.read() {
        match event {
            ScoreEvent::Player1 => score.p1 += 1,
            ScoreEvent::Player2 => score.p2 += 1,
        }
    }
}