use crate::{
    resources::{prefabs::UiPrefabRegistry, Paused, QuitToMenu, ResourceRegistry},
    states::OptionsState,
    utils::delete_hierarchy,
};
use amethyst::{
    ecs::Entity,
    input::{self, VirtualKeyCode},
    prelude::*,
    ui::{UiEvent, UiEventType, UiFinder},
};

const PAUSE_ID: &str = "pause";

const RETURN_TO_GAME_BUTTON: &str = "return_to_game";
const OPTIONS_BUTTON: &str = "options";
const MENU_BUTTON: &str = "return_to_menu";
const EXIT_BUTTON: &str = "exit_game";

#[derive(Default)]
pub struct PauseState {
    root_entity: Option<Entity>,
    return_to_game_button: Option<Entity>,
    options_button: Option<Entity>,
    menu_button: Option<Entity>,
    exit_button: Option<Entity>,
}

impl SimpleState for PauseState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        *data.world.write_resource::<Paused>() = Paused::Paused;
        self.init_gui(data);
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        *data.world.write_resource::<Paused>() = Paused::Unpaused;
        self.deinit_gui(data);
    }

    fn on_pause(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.deinit_gui(data);
    }

    fn on_resume(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.init_gui(data);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if self.options_button.map_or(false, |button| button == target) {
                    Trans::Push(Box::new(OptionsState::default()))
                } else if self.menu_button.map_or(false, |button| button == target) {
                    *data.world.write_resource::<QuitToMenu>() = QuitToMenu(true);
                    Trans::Pop
                } else if self.exit_button.map_or(false, |button| button == target) {
                    Trans::Quit
                } else if self
                    .return_to_game_button
                    .map_or(false, |button| button == target)
                {
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            StateEvent::Window(event) => {
                if input::is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }
}
impl PauseState {
    fn init_gui(&mut self, data: StateData<GameData>) {
        let prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, PAUSE_ID)
            .expect("Couldn't load pause menu prefab");
        self.root_entity = Some(data.world.create_entity().with(prefab).build());
        data.data.update(&data.world);

        data.world.exec(|ui_finder: UiFinder<'_>| {
            self.return_to_game_button = ui_finder.find(RETURN_TO_GAME_BUTTON);
            self.options_button = ui_finder.find(OPTIONS_BUTTON);
            self.menu_button = ui_finder.find(MENU_BUTTON);
            self.exit_button = ui_finder.find(EXIT_BUTTON);
        });
    }
    fn deinit_gui(&mut self, mut data: StateData<GameData>) {
        if let Some(e) = self.root_entity.take() {
            delete_hierarchy(&mut data.world, e);
            self.options_button = None;
            self.menu_button = None;
            self.exit_button = None;
        }
    }
}
