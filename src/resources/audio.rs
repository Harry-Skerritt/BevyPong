use bevy::prelude::*;


#[derive(Resource)]
pub struct AudioAssets {
    pub paddle_hit: Handle<AudioSource>,
    pub score_point: Handle<AudioSource>,
    pub win_sound: Handle<AudioSource>,
}