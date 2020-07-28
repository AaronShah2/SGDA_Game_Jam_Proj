// neccesary imports
use amethyst::{
    core::transform::Transform, // position?
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode}, // input?
    prelude::*,
    renderer::Camera,         // graphics & rendering tools?
    window::ScreenDimensions, // resolution?
};

use crate::resources::sprites::SpriteSheetRegister;

use log::info;

/// Testing game state
#[derive(Default)]
pub struct TestState;

const SHEET_ID: &str = "test-1";

impl SimpleState for TestState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // Screen dimmensions to initialize Camera
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(data.world, &dimensions);

        // Place sprites
        init_sprites(data.world, &dimensions);
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

/// Creates Camera in world
/// 'dimmensions' centers camera around screen
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.); // Centers Camera

    world
        .create_entity() // creates Camera entity?
        .with(Camera::standard_2d(dimensions.width(), dimensions.height())) // creates 2d Camera centered on screen
        .with(transform) // updates camera postion to be centered on screen
        .build();
}

fn init_sprites(world: &mut World, dimensions: &ScreenDimensions) {
    // Bounds are currently hard-coded
    for i in 0..1 {
        // Center our sprites around the center of the window
        let x = (i as f32 - 1.) * 100. + dimensions.width() * 0.5;
        let y = (i as f32 - 1.) * 100. + dimensions.height() * 0.5;
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.);

        // Create an entity for each sprite and attach the `SpriteRender` as
        // well as the transform. If you want to add behaviour to your sprites,
        // you'll want to add a custom `Component` that will identify them, and a
        // `System` that will iterate over them. See https://book.amethyst.rs/stable/concepts/system.html
        let sprite_render = world
            .read_resource::<SpriteSheetRegister>()
            .find_sprite(world, SHEET_ID, i)
            .expect("Couldn't load sprite");
        world
            .create_entity()
            .with(sprite_render)
            .with(transform)
            .build();
    }
}
