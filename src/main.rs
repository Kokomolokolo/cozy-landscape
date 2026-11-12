use bevy::{anti_alias::taa::TemporalAntiAliasPlugin, prelude::*};

use crate::{camera::CameraPlugin, preloader::PreloaderPlugin, world::WorldPlugin};

mod camera;
mod world;
mod preloader;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    InGame,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        //.add_plugins(TemporalAntiAliasPlugin)
        // .insert_resource(Msaa::Sample4)
        .init_state::<AppState>()
        .add_plugins((
            PreloaderPlugin, 
            CameraPlugin, 
            WorldPlugin
        ))
        .add_systems(Startup, setup)
        .run();
}

pub struct Pos(
    IVec2
);


fn setup(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0, // Helligkeit der Sonne
            // shadow_maps_enabled: true,
            ..default()
        },
        Transform::from_translation(Vec3 { x: 40.0, y: 80.0, z: 40.0 }),
    ));
    commands.insert_resource(ClearColor(Color::srgb(0.3, 0.4, 0.99)));
}