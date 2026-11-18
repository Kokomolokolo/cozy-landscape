use std::collections::HashMap;

use bevy::prelude::*;

use crate::AppState;

pub struct PreloaderPlugin;

impl Plugin for PreloaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, preload_models);
        app.add_systems(OnEnter(AppState::InGame), spawn_test_asset);
        app.add_systems(Update, fix_tree_materials_once);
    }
}

#[derive(Resource, Debug)]
pub struct GameAssets {
    pub assets: HashMap<String, Handle<WorldAsset>>
}

const MODEL_NAMES: &[&str] = &[
    "tree_pack.glb",
    "tree_large_0.glb",
    "tree_large_1.glb",
    "tree_large_2.glb",
    "tree_large_3.glb",
    "low_grass_0.glb",
    "low_grass_1.glb",
    "low_grass_2.glb",
    "low_grass_3.glb",
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
            game_assets.assets.get("tree_large_0.glb").unwrap().clone()
        ),
        Transform::from_xyz(0.0, -0.4, 0.0)
            .with_scale(Vec3::splat(1.0)),
    ));
}

// Das hat sehr viel gebracht
pub fn fix_tree_materials_once(
    mut materials: ResMut<Assets<StandardMaterial>>,
    // Wir triggern, sobald neue StandardMaterials hinzugefügt werden
    mut added_materials: MessageReader<AssetEvent<StandardMaterial>>,
) {
    for event in added_materials.read() {
        if let AssetEvent::Added { id } = event {
            if let Some(mut material) = materials.get_mut(*id) {
                // Wenn das Material eine Textur hat (typisch für Blätter/Rinde)
                if material.base_color_texture.is_some() {
                    material.alpha_mode = AlphaMode::Mask(0.5);
                    material.double_sided = true; // Hilft oft bei dünnen Blättern
                }
            }
        }
    }
}