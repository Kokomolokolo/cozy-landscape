use std::collections::HashMap;

use bevy::prelude::*;

use crate::AppState;

pub struct PreloaderPlugin;

impl Plugin for PreloaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, preload_models);
        app.add_systems(OnEnter(AppState::InGame), spawn_test_asset);
    }
}

#[derive(Resource, Debug)]
pub struct GameAssets {
    pub assets: HashMap<String, Handle<WorldAsset>>
}

const MODEL_NAMES: &[&str] = &[
    "tree_pack.glb",
];

fn preload_models(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AppState>>
) {
    let mut assets = HashMap::new();

    for file in MODEL_NAMES {
        assets.insert(
            file.to_string(), 
            asset_server.load(
                GltfAssetLabel::Scene(0).from_asset(format!("models/{file}"))
            )
        );
    }

    commands.insert_resource(GameAssets { assets });
    info!("Loaded all models succesfully");
    next_state.set(AppState::InGame);
}

fn spawn_test_asset(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    println!("Test2");
    commands.spawn((
        WorldAssetRoot(
            game_assets.assets.get("tree_pack.glb").unwrap().clone()
        ),
        Transform::from_xyz(0.0, -0.4, 0.0)
            .with_scale(Vec3::splat(1.0)),
    ));
}