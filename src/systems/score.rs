use bevy::prelude::*;
use crate::events::score_events::ScoreEvent;
use crate::resources::Score;
use crate::resources::score::WinTimer;
use crate::states::game_state::GameState;

pub fn update_score(
    mut reader: MessageReader<ScoreEvent>,
    mut score: ResMut<Score>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
) {
    for event in reader.read() {
        match event {
            ScoreEvent::Player1 => score.p1 += 1,
            ScoreEvent::Player2 => score.p2 += 1,
        }

        let winner =
            if score.p1 >= score.win_score {
                Some(1)
            } else if score.p2 >= score.win_score {
                Some(2)
            } else {
                None
            };

        if let Some(player_id) = winner {
            next_state.set(GameState::WinnerScreen);

            commands.insert_resource(WinTimer {
                timer: Timer::from_seconds(3.0, TimerMode::Once),
                winner: player_id,
            });
        }
    }
}