use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};

#[derive(Component, Clone, Copy, Default)]
pub struct HudFps;
#[derive(Component, Clone, Copy, Default)]
pub struct HudChunks;
#[derive(Component, Clone, Copy, Default)]
pub struct HudCoords;



pub fn setup_hud(mut commands: Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute, 
            left: px(5), 
            top: px(5), 
            padding: UiRect::all(px(3)), 
            display: Display::Flex, 
            flex_direction: FlexDirection::Column, 
            row_gap: px(2),
            ..default()
        },
    ))
    .with_children(|parent|  {
        let font_size = 12.;
        parent.spawn((
            Text::new("FPS"),
            TextFont {
                font_size: FontSize::Px(font_size),
                ..default()
            },
            TextColor::WHITE,
            HudFps
        ));
    });
}

pub fn update_hud(
    mut texts: ParamSet<(
        Query<&mut Text, With<HudFps>>,
    )>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.smoothed())
        .unwrap_or(-0.1);

    for mut text in &mut texts.p0() {
        *text = Text::new(format!("FPS: {:.0}", fps));
    }
}