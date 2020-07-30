use crate::{
    resources::{prefabs::UiPrefabRegistry, Controls, ResourceRegistry},
    utils::delete_hierarchy,
};

use amethyst::{
    ecs::Entity,
    prelude::*,
    ui::{UiEvent, UiEventType, UiFinder, UiText},
};

const ROOT_ID: &str = "options";
const BACK_BUTTON_ID: &str = "back";
const CONTROLS_BUTTON_ID: &str = "controls";
const CONTROLS_LABEL_ID: &str = "controls_label";

#[derive(Default)]
pub struct OptionsState {
    root_entity: Option<Entity>,
    controls_button: Option<Entity>,
    controls_label: Option<Entity>,
    back_button: Option<Entity>,
}

impl SimpleState for OptionsState {
    // handles button presses
    fn handle_event(
        &mut self,
        mut data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Ui(UiEvent {
                event_type: UiEventType::Click,
                target,
            }) => {
                if self
                    .back_button
                    .map_or(false, |back_button| back_button == target)
                {
                    Trans::Pop
                } else if self
                    .controls_button
                    .map_or(false, |button| button == target)
                {
                    let mut controls = *data
                        .world
                        .entry::<Controls>()
                        .or_insert_with(Default::default);
                    controls = controls.successor();
                    controls.set_control_scheme(&mut data.world);
                    if let Some(label) = self.controls_label {
                        data.world
                            .write_storage::<UiText>()
                            .get_mut(label)
                            .expect("Couldn't find UiText on Controls Button Label")
                            .text = controls.get_button_label().to_string();
                    }
                    data.world.insert(controls);
                    Trans::None
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.load_ui(&mut data);
    }

    fn on_stop(&mut self, mut data: StateData<GameData>) {
        self.tear_down_ui(&mut data);
    }
}

impl OptionsState {
    fn load_ui(&mut self, data: &mut StateData<GameData>) {
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
            self.controls_button = ui_finder.find(CONTROLS_BUTTON_ID);
            self.controls_label = ui_finder.find(CONTROLS_LABEL_ID);
        });
        let controls = *data
            .world
            .entry::<Controls>()
            .or_insert_with(Default::default);
        if let Some(label) = self.controls_label {
            data.world
                .write_storage::<UiText>()
                .get_mut(label)
                .expect("Couldn't find UiText on Controls Button Label")
                .text = controls.get_button_label().to_string();
        }
    }

    fn tear_down_ui(&mut self, data: &mut StateData<GameData>) {
        if let Some(e) = self.root_entity.take() {
            delete_hierarchy(&mut data.world, e);
            self.back_button = None;
            self.controls_button = None;
            self.controls_label = None;
        }
    }
}
