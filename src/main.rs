use avian3d::PhysicsPlugins;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

use crate::{camera::CameraPlugin, gui::GUIPlugin, player::PlayerPlugin, preloader::PreloaderPlugin, vegetation::VegetationPlugin, world::{CHUNK_SIZE, WorldPlugin}};

mod camera;
mod world;
mod preloader;
mod gui;
mod player;
mod vegetation;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    InGame,
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin::default(),
            PhysicsPlugins::default(),
        ))
        //.add_plugins(TemporalAntiAliasPlugin)
        // .insert_resource(Msaa::Sample4)
        .init_state::<AppState>()
        .add_plugins((
            PreloaderPlugin, 
            // CameraPlugin, 
            WorldPlugin,
            GUIPlugin,
            VegetationPlugin,
            PlayerPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

// COMPONETS
// weil ich zu faul bin da was eigenens zu machen
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkPos(pub IVec2);

impl ChunkPos {
    pub fn to_world(&self) -> Vec2 {
        Vec2::new(
            (self.0.x * CHUNK_SIZE as i32) as f32, 
            (self.0.y * CHUNK_SIZE as i32) as f32, 
        )
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 1000.0, // Helligkeit der Sonne
            shadow_maps_enabled: true,
            
            ..default()
        },
        Transform::from_translation(Vec3 { x: 40.0, y: 80.0, z: 40.0 }).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.insert_resource(ClearColor(Color::srgb(0.3, 0.4, 0.99)));
}


// https://chatgpt.com/c/6afa02ca-1ca4-83eb-98ef-3b195fcfd0ad