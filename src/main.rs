// This file drives the game

// neccesary Amethest imports
use amethyst::{
    assets::PrefabLoaderSystemDesc,
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

pub mod components;
pub mod resources;
pub mod states;
pub mod systems;
pub mod utils;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets_dir = app_root.join("assets");
    let config_dir = app_root.join("config");
    let display_config_path = config_dir.join("display.ron");

    // adds key bindings & creates input handle
    let bindings_config = config_dir.join("inputs.ron");
    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_config)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(input_bundle)?
        .with_bundle(TransformBundle::new())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(AudioBundle::default())?
        // testing out key configures
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default()),
        )?
        .with_system_desc(
            PrefabLoaderSystemDesc::<resources::prefabs::CharacterPrefab>::default(),
            "character_prefab_loader",
            &[],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<resources::prefabs::BackgroundPrefab>::default(),
            "background_prefab_loader",
            &[],
        )
        .with_system_desc(
            PrefabLoaderSystemDesc::<resources::prefabs::ObstaclePrefab>::default(),
            "obstacle_prefab_loader",
            &[],
        )
        .with(systems::PlayerSystem, "player_system", &["input_system"])
        .with(
            systems::PlayerCollisionSystem,
            "player_col_system",
            &["player_system"],
        )
        .with(
            systems::ScoreTrackingSystem,
            "score_system",
            &["player_system"],
        )
        .with(
            systems::BackgroundRepeatSystem,
            "background_repeat_system",
            &["player_system"],
        )
        .with(
            systems::ObstacleRandomizationSystem,
            "obstacle_randomization_system",
            &["player_system"]
        )
        .with(
            systems::EnemyMovementSystem,
            "enemy_movement_system",
            &["player_system"],
        )
        .with(
            systems::EnemyCollisionSystem,
            "enemy_collision_system",
            &["player_system", "enemy_movement_system"],
        )
        .with(
            systems::DogCollisionSystem,
            "dog_collision_system",
            &["player_system"],
        )
        .with(
            systems::DogAttackSystem,
            "dog_attack_system",
            &["player_system"],
        )
        .with(systems::MudSystem, "mud_system", &["player_system"])
        .with(systems::DogSystem, "dog_system", &["player_system"]);

    let mut game = Application::new(assets_dir, states::LoadingState::default(), game_data)?;
    game.run();

    Ok(())
}
