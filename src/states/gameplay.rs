// neccesary imports
use crate::{
    resources::{
        prefabs::{CharacterPrefabRegistry, ObstaclePrefabRegistry},
        sprites::SpriteSheetRegister, QuitToMenu,
        ResourceRegistry,
    },
    states::PauseState,
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
    mud: Option<Entity>,
}

const PLAYER_SHEET_ID: &str = "Gamer";
const ENEMY_SHEET_ID: &str = "walkRight";
const MUD_SHEET_ID: &str = "mud";

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.init_player(data.world);
        self.init_enemy(data.world);
        self.init_mud(data.world);
        data.world.insert(QuitToMenu(false));
    }

    fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.deinit_sprites(&mut data.world);
    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) {
                return Trans::Quit;
            }
            // Check if the player presses escape
            if is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Push(Box::new(PauseState::default()));
            }

            // Listen to any key events
            if let Some(_event) = get_key(&event) {
                // log::info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        if *data.world.read_resource::<QuitToMenu>() == QuitToMenu(true) {
            *data.world.write_resource::<QuitToMenu>() = QuitToMenu(false);
            return Trans::Pop;
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
        if let Some(mud) = self.mud.take() {
            delete_hierarchy(world, mud);
        }
    }

    fn init_mud(&mut self, world: &mut World) {
        let sprite_render = world
            .read_resource::<SpriteSheetRegister>()
            .find_sprite(world, MUD_SHEET_ID, 0)
            .unwrap();
        let mud_prefab = world
            .read_resource::<ObstaclePrefabRegistry>()
            .find(world, "mud")
            .expect("Couldn't find player prefab");
        self.mud = Some(
            world
                .create_entity()
                .with(sprite_render)
                .with(mud_prefab)
                .build(),
        );
    }
}
