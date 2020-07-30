use crate::{
    resources::{prefabs::UiPrefabRegistry, Paused, ResourceRegistry},
    utils::delete_hierarchy,
};
use amethyst::{
    ecs::Entity,
    input::{self, VirtualKeyCode},
    prelude::*,
};

const PAUSE_ID: &str = "pause";

#[derive(Default)]
pub struct PauseState {
    root_entity: Option<Entity>,
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

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if input::is_key_down(&event, VirtualKeyCode::Escape) {
                Trans::Pop
            } else {
                Trans::None
            }
        } else {
            Trans::None
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
    }
    fn deinit_gui(&mut self, mut data: StateData<GameData>) {
        if let Some(e) = self.root_entity.take() {
            delete_hierarchy(&mut data.world, e);
        }
    }
}
