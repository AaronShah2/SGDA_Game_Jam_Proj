use crate::{
    resources::{prefabs::UiPrefabRegistry, ResourceRegistry},
    states::GameplayState,
    utils::delete_hierarchy,
};

use amethyst::{
    ecs::Entity,
    prelude::*,
    ui::{UiEvent, UiEventType, UiFinder},
};

const GAME_OVER_ID: &str = "game-over";

const PLAY_AGAIN_BUTTON: &str = "play_again";
const RETURN_TO_MENU_BUTTON: &str = "return_to_menu";

#[derive(Default)]
pub struct GameOverState {
    root_entity: Option<Entity>,
    play_again_button: Option<Entity>,
    return_to_menu_button: Option<Entity>,
}

impl SimpleState for GameOverState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.init_gui(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.deinit_gui(&mut data);
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if self
                    .play_again_button
                    .map_or(false, |button| button == target)
                {
                    Trans::Switch(Box::new(GameplayState::default()))
                } else if self
                    .return_to_menu_button
                    .map_or(false, |button| button == target)
                {
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }
}

impl GameOverState {
    fn init_gui(&mut self, data: &mut StateData<GameData>) {
        let menu_prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, GAME_OVER_ID)
            .expect("Couldn't load game over prefab");
        self.root_entity = Some(data.world.create_entity().with(menu_prefab).build());
        data.data.update(data.world);
        data.world.exec(|ui_finder: UiFinder<'_>| {
            self.play_again_button = ui_finder.find(PLAY_AGAIN_BUTTON);
            self.return_to_menu_button = ui_finder.find(RETURN_TO_MENU_BUTTON);
        });
    }

    fn deinit_gui(&mut self, data: &mut StateData<GameData>) {
        if let Some(e) = self.root_entity.take() {
            delete_hierarchy(data.world, e);
            self.play_again_button = None;
            self.return_to_menu_button = None;
        }
        data.data.update(data.world);
    }
}
