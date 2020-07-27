use crate::{
    resources::{prefabs::UiPrefabRegistry, ResourceRegistry},
    states::Test,
};
use amethyst::{
    ecs::Entity,
    prelude::*,
    ui::{UiEvent, UiEventType, UiFinder},
};

const MENU_ID: &str = "menu";

#[derive(Default)]
pub struct MenuState {
    start_button: Option<Entity>,
    options_button: Option<Entity>,
    exit_button: Option<Entity>,
    root_entity: Option<Entity>,
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let menu_prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, MENU_ID)
            .expect("Couldn't load menu prefab");
        self.root_entity = Some(data.world.create_entity().with(menu_prefab).build());
        data.data.update(&data.world);
        data.world.exec(|ui_finder: UiFinder<'_>| {
            self.start_button = ui_finder.find("start");
            self.options_button = ui_finder.find("options");
            self.exit_button = ui_finder.find("exit");
        });
    }

    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent,) -> SimpleTrans {
        match event {
            StateEvent::Ui(UiEvent { event_type: UiEventType::Click, target, }) => {
                if self.start_button.map_or(false, |button| button == target) {
                    Trans::Push(Box::new(Test::default()))
                } else if self.options_button.map_or(false, |button| button == target) {
                    unimplemented!("Options aren't implemented yet :(")
                } else if self.exit_button.map_or(false, |button| button == target) {
                    Trans::Quit
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn on_pause(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        // TODO Hide the Ui elements
    }

    fn on_resume(&mut self, _data: StateData<'_, GameData<'_, '_>>) {
        // TODO Unhide the Ui elements
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        let to_delete: Vec<Entity> = self.root_entity.into_iter().chain(self.start_button.into_iter()).chain(self.options_button.into_iter()).chain(self.exit_button.into_iter()).collect();
        data.world.delete_entities(&to_delete).expect("Failed to remove menu elements");
        self.root_entity = None;
        self.start_button = None;
        self.options_button = None;
        self.exit_button = None;
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        Trans::None
    }
}
