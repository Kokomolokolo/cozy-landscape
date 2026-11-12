use bevy::prelude::*;

use crate::camera::CameraPlugin;

mod camera;
mod world;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    InGame,
}

fn main() {
    App::new()
        .init_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraPlugin)
        .add_systems(Startup, setup)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn((
        PointLight {
            intensity: 10000.,
            ..default()
        }
    ));
}