// neccesary imports
use crate::{
    resources::{
        prefabs::{BackgroundPrefabRegistry, CharacterPrefabRegistry},
        sprites::SpriteSheetRegister,
        ResourceRegistry,
    },
    utils::delete_hierarchy,
};
use amethyst::{
    ecs::Entity,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode}, // input?
    prelude::*,
};

/// Testing game state
#[derive(Default)]
pub struct GameplayState {
    player: Option<Entity>,
    enemy: Option<Entity>,
    background: Option<Entity>,
}

const PLAYER_SHEET_ID: &str = "Gamer";
const ENEMY_SHEET_ID: &str = "walkRight";
const BG_SHEET_ID: &str = "BG";

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.init_player(data.world);
        self.init_enemy(data.world);
        self.init_bg(data.world);
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
            if is_close_requested(&event) {
                return Trans::Quit;
            }
            // Check if the player presses escape
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Pop;
            }

            // Listen to any key events
            if let Some(_event) = get_key(&event) {
                // log::info!("handling key event: {:?}", event);
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
    fn init_player(&mut self, world: &mut World) {
        let sprite_render = world
            .read_resource::<SpriteSheetRegister>()
            .find_sprite(world, PLAYER_SHEET_ID, 0)
            .unwrap();
        let player_prefab = world
            .read_resource::<CharacterPrefabRegistry>()
            .find(world, "player")
            .expect("Couldn't find player prefab");
        self.player = Some(
            world
                .create_entity()
                .with(sprite_render)
                .with(player_prefab)
                .build(),
        );
    }

    fn init_enemy(&mut self, world: &mut World) {
        let sprite_render = world
            .read_resource::<SpriteSheetRegister>()
            .find_sprite(world, ENEMY_SHEET_ID, 0)
            .unwrap_or_else(|| panic!("Couldn't find spritesheet {}", ENEMY_SHEET_ID));
        let enemy_prefab = world
            .read_resource::<CharacterPrefabRegistry>()
            .find(world, "enemy")
            .expect("Couldn't find enemy prefab");
        self.enemy = Some(
            world
                .create_entity()
                .with(sprite_render)
                .with(enemy_prefab)
                .build(),
        );
    }

    fn deinit_sprites(&mut self, world: &mut World) {
        if let Some(player) = self.player.take() {
            delete_hierarchy(world, player);
        }
        if let Some(enemy) = self.enemy.take() {
            delete_hierarchy(world, enemy);
        }
        if let Some(background) = self.background.take() {
            delete_hierarchy(world, background);
        }
    }

    fn init_bg(&mut self, world: &mut World) {
        let sprite_render = world
            .read_resource::<SpriteSheetRegister>()
            .find_sprite(world, BG_SHEET_ID, 0)
            .unwrap_or_else(|| panic!("Couldn't find spritesheet {}", BG_SHEET_ID));
        let background_prefab = world
            .read_resource::<BackgroundPrefabRegistry>()
            .find(world, "background")
            .expect("Couldn't find background prefab");
        self.background = Some(
            world
                .create_entity()
                .with(sprite_render)
                .with(background_prefab)
                .build(),
        );
    }
}
