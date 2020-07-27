use amethyst::{
    ecs::Entity,
    prelude::*,
    ui::{UiEvent, UiEventType, UiFinder},
};
use crate::resources::prefabs;

const MENU: &str = "menu";

#[derive(Default)]
pub struct MenuState {
    start_button: Option<Entity>,
    options_button: Option<Entity>,
    exit_button: Option<Entity>,
    root_entity: Option<Entity>,
}

impl SimpleState for MenuState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Creating the menu");
        let menu = data.world.read_resource::<prefabs::UiPrefabRegistry>().find(data.world, MENU).expect("Couldn't load menu prefab");
        self.root_entity = Some(data.world.create_entity().with(menu).build());
        data.data.update(&data.world);
        data.world.exec(|ui_finder: UiFinder<'_>| {
            self.start_button = ui_finder.find("start");
            self.options_button = ui_finder.find("controls");
            self.exit_button = ui_finder.find("exit");
        });
    }
}
