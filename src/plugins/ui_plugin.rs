use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_score);
    }
}

#[derive(Component)]
struct Player1Score;

#[derive(Component)]
struct Player2Score;

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