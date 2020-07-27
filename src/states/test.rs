// neccesary imports
use amethyst::{
    assets::{AssetStorage, Handle, Loader}, // assets?
    core::transform::Transform, // position?
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode}, // input?
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture}, // graphics & rendering tools?
    window::ScreenDimensions, // resolution?
};

use log::info;

/// Testing game state
pub struct Test;

impl SimpleState for Test {

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Screen dimmensions to initialize Camera
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        let sprites = load_sprite_sheet(world, "test-1", "jpeg");
        init_sprites(world, sprites.clone(), &dimensions);
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

/// Loads a SpriteSheet with the given name and extension, and the
/// corresponding .ron file to interpret it.
///
/// Name should be a relative path from the sprite directory.
fn load_sprite_sheet(world: &mut World, name: &str, ext: &str) -> Handle<SpriteSheet> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let loader = world.read_resource::<Loader>();
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("sprites/{}.{}", name, ext),
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("sprites/{}.ron", name),
        SpriteSheetFormat(texture_handle),
        (),
        &sprite_sheet_store,
    )
}

fn init_sprites(world: &mut World, sheet: Handle<SpriteSheet>, dimensions: &ScreenDimensions) {
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
        world
            .create_entity()
            .with(SpriteRender { sprite_sheet: sheet.clone(), sprite_number: i })
            .with(transform)
            .build();
    }
}
