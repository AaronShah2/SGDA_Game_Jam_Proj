use crate::resources::{ResourceRegistry, prefabs::UiPrefabRegistry};

use amethyst::{core::transform::ParentHierarchy, ecs::Entity, prelude::*, ui::{UiEvent, UiEventType, UiFinder}};

const ROOT_ID: &str = "options";
const BACK_BUTTON_ID: &str = "back";

#[derive(Default)]
pub struct OptionsState {
    root_entity: Option<Entity>,
    back_button: Option<Entity>,
}

impl SimpleState for OptionsState {
    // handles button presses
    fn handle_event(&mut self, _data: StateData<'_, GameData<'_, '_>>, event: StateEvent,) -> SimpleTrans {
        match event {
            StateEvent::Ui(UiEvent { event_type: UiEventType::Click, target, }) => {
                if self.back_button.map_or(false, |back_button| back_button == target) {
                    Trans::Pop
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        self.load_ui(data);
    }

    fn on_stop(&mut self, data: StateData<GameData>) {
        self.tear_down_ui(data);
    }
}

impl OptionsState {
    fn load_ui(&mut self, data: StateData<GameData>) {
        // Instantiate the menu
        let menu_prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, ROOT_ID)
            .expect("Couldn't load menu prefab");
        self.root_entity = Some(data.world.create_entity().with(menu_prefab).build());
        data.data.update(&data.world);
        // Assign important entities to member variables
        data.world.exec(|ui_finder: UiFinder<'_>| {
            self.back_button = ui_finder.find(BACK_BUTTON_ID);
        });
    }

    fn tear_down_ui(&mut self, data: StateData<GameData>) {
        match self.root_entity {
            Some(e) => {
                let mut to_delete: Vec<Entity> = data.world.read_resource::<ParentHierarchy>().all_children_iter(e).collect();
                to_delete.push(e);
                data.world.delete_entities(&to_delete).expect("Failed to remove menu elements");
                self.root_entity = None;
                self.back_button = None;
            },
            None => {},
        }
    }
}
