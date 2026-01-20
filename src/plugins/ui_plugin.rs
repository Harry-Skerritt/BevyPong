use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
    ecs::system::ParamSet,
};
use crate::components::score::*;
use crate::resources::Score;


pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_score)
            .add_systems(Startup, setup_fps);

        app
            .add_systems(Update, update_score_ui)
            .add_systems(Update, fps_update);

    }
}



fn setup_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            top: px(20.0),
            left: px(0.0),
            width: percent(100.0),
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::NONE)
    ))
    .with_children(|parent| {
        parent.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::NONE)
        ))
        .with_children(|row| {
            // Player 1
            row.spawn((
                Text::new("0"),
                TextFont {
                    font: asset_server.load("fonts/PixelifySans-Bold.ttf").into(),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb_u8(255, 138, 130)),
                Node {
                    margin: UiRect::right(px(60.0)),
                    ..default()
                },
                Player1Score,
            ));

            row.spawn((
                Text::new("0"),
                TextFont {
                    font: asset_server.load("fonts/PixelifySans-Bold.ttf").into(),
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::srgb_u8(130, 143, 255)),
                Node {
                    margin: UiRect::left(px(60.0)),
                    ..default()
                },
                Player2Score,
            ));
        });
    });
}


fn update_score_ui(
    score: Res<Score>,
    mut texts: ParamSet<(
        Query<&mut Text, With<Player1Score>>,
        Query<&mut Text, With<Player2Score>>,
    )>,
) {
    if !score.is_changed() {
        return;
    }

    if let Ok(mut text1) = texts.p0().single_mut() {
        text1.clear();
        text1.push_str(&score.p1.to_string());
    }

    if let Ok(mut text2) = texts.p1().single_mut() {
        text2.clear();
        text2.push_str(&score.p2.to_string());
    }
}


// --- FPS ----
#[derive(Component)]
struct FpsText;

fn setup_fps(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Text::new("FPS: "),
        TextFont {
            font: asset_server.load("fonts/OpenSans-Bold.ttf").into(),
            font_size: 15.0,
            ..default()
        },
    ))
    .with_child((
        TextSpan::default(),
        (
            TextFont {
                font: asset_server.load("fonts/OpenSans-Bold.ttf").into(),
                font_size: 15.0,
                ..default()
            },
        ),
        FpsText,
    ));
}

fn fps_update(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut TextSpan, With<FpsText>>,
) {
    for mut span in &mut query {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS)
            && let Some(value) = fps.smoothed()
        {
            // Update the value of the second section
            **span = format!("{value:.2}");
        }
    }
}