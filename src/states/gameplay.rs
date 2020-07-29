// neccesary imports
use amethyst::{
    core::transform::Transform, // position?
    ecs::Entity,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode}, // input?
    prelude::*,
    renderer::Camera,         // graphics & rendering tools?
    window::ScreenDimensions, // resolution?
};

use crate::{
    resources::{prefabs::CharacterPrefabRegistry, sprites::SpriteSheetRegister, ResourceRegistry},
    utils::delete_hierarchy,
};

use log::info;

/// Testing game state
#[derive(Default)]
pub struct GameplayState {
    player: Option<Entity>,
}

const SHEET_ID: &str = "Gamer";

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // Screen dimmensions to initialize Camera
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        self.init_camera(data.world, &dimensions);

        // Places play
        self.init_player(data.world, &dimensions);
    }

    fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.deinit_sprites(&mut data.world);
    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

impl GameplayState {
    /// Creates Camera in world
    /// 'dimmensions' centers camera around screen
    fn init_camera(&self, world: &mut World, dimensions: &ScreenDimensions) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.); // Centers Camera

        world
            .create_entity() // creates Camera entity?
            .with(Camera::standard_2d(dimensions.width(), dimensions.height())) // creates 2d Camera centered on screen
            .with(transform) // updates camera postion to be centered on screen
            .build();
    }

    fn init_player(&mut self, world: &mut World, dimensions: &ScreenDimensions) {
        // Center our sprites around the center of the window
        let x = dimensions.width() * 0.5;
        let y = dimensions.height() * 0.5;
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.);

        let sprite_render = world
            .read_resource::<SpriteSheetRegister>()
            .find_sprite(world, SHEET_ID, 0)
            .unwrap();
        let player_prefab = world
            .read_resource::<CharacterPrefabRegistry>()
            .find(world, "player")
            .expect("Couldn't find player prefab");
        self.player = Some(
            world
                .create_entity()
                .with(sprite_render)
                .with(transform)
                .with(player_prefab)
                .build(),
        );
    }

    fn deinit_sprites(&mut self, world: &mut World) {
        if let Some(player) = self.player {
            delete_hierarchy(world, player);
            self.player = None;
        }
    }
}
