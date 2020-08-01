use crate::{
    resources::{prefabs::UiPrefabRegistry, ResourceRegistry},
    states::GameplayState,
    utils,
};
use amethyst::{
    core::transform::ParentHierarchy,
    ecs::Entity,
    input::{self, VirtualKeyCode},
    prelude::*,
    ui::{UiText, UiTransform},
};

const DIALOG_LINE_TIME: f64 = 1.0;

const IRC_DIALOG: [(&str, &str, &str, [f32; 4]); 7] = [
    (
        "14:32:13",
        "Avaritia",
        "PLACEHOLDER",
        [0.0, 0.676, 0.0, 1.0],
    ),
    ("14:32:14", "AeonSlayer1979", "yeah", [0.676, 0.0, 0.0, 1.0]),
    ("14:32:14", "AeonSlayer1979", "hey", [0.676, 0.0, 0.0, 1.0]),
    (
        "14:32:14",
        "AeonSlayer1979",
        "wanna meet irl",
        [0.676, 0.0, 0.0, 1.0],
    ),
    ("14:32:14", "Avaritia", "sure", [0.0, 0.676, 0.0, 1.0]),
    ("14:32:15", "Avaritia", "where at", [0.0, 0.676, 0.0, 1.0]),
    (
        "14:32:15",
        "AeonSlayer1979",
        "you can come over to my place",
        [0.676, 0.0, 0.0, 1.0],
    ),
];

const IRC_ROW_ID: &str = "irc_line";

const TIMESTAMP_LABEL: &str = "timestamp";
const AUTHOR_LABEL: &str = "author";
const MESSAGE_LABEL: &str = "message";

#[derive(Default)]
pub struct CutsceneState {
    time: f64,
    dialog_number: usize,
    entites: Vec<Entity>,
    to_update: bool,
}

impl SimpleState for CutsceneState {
    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Window(event) => {
                if input::is_key_down(&event, VirtualKeyCode::Escape) {
                    Trans::Switch(Box::new(GameplayState::default()))
                } else {
                    Trans::None
                }
            }
            _ => Trans::None,
        }
    }

    fn fixed_update(&mut self, mut data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        self.time += 1.0 / 60.0;
        if self.time > DIALOG_LINE_TIME * self.dialog_number as f64 {
            if self.dialog_number < 7 {
                let (time, speaker, line, author_color) = IRC_DIALOG[self.dialog_number];
                self.render_irc_dialog(&mut data, time, speaker, line, author_color);
            }
            self.dialog_number += 1;
        }
        data.data.update(&data.world);
        Trans::None
    }

    fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.deinit_entities(&mut data.world);
    }
}

impl CutsceneState {
    /// Despawn all entities produced by this
    fn deinit_entities(&mut self, world: &mut World) {
        for entity in self.entites.drain(..) {
            utils::delete_hierarchy(world, entity);
        }
    }

    /// Display a new line of IRC
    fn render_irc_dialog(
        &mut self,
        data: &mut StateData<GameData>,
        time: &str,
        speaker: &str,
        line: &str,
        author_color: [f32; 4],
    ) {
        let prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, IRC_ROW_ID)
            .expect("Couldn't fiund prefab for IRC line");
        let new_line = data.world.create_entity().with(prefab.clone()).build();
        data.data.update(&data.world);
        self.to_update = true;
        // Change the text and colors to match this specific line
        let uitext_storage = &mut data.world.write_storage::<UiText>();
        let num_descendants = data
            .world
            .read_resource::<ParentHierarchy>()
            .all_children_iter(new_line)
            .map(|entity| {
                match data
                    .world
                    .read_storage::<UiTransform>()
                    .get(entity)
                    .map(|e| &e.id)
                {
                    Some(label) if label == TIMESTAMP_LABEL => {
                        let text = uitext_storage
                            .get_mut(entity)
                            .expect("Timestamp has no UiText");
                        text.text = time.to_string();
                    }
                    Some(label) if label == AUTHOR_LABEL => {
                        let text = uitext_storage
                            .get_mut(entity)
                            .expect("Timestamp has no UiText");
                        text.text = speaker.to_string();
                        text.color = author_color;
                    }
                    Some(label) if label == MESSAGE_LABEL => {
                        let text = uitext_storage
                            .get_mut(entity)
                            .expect("Timestamp has no UiText");
                        text.text = line.to_string();
                    }
                    label => log::info!("Did nothing to entity {:?}", label),
                }
            })
            .count();
        let prefab_storage = data
            .world
            .read_resource::<amethyst::assets::AssetStorage<amethyst::ui::UiPrefab>>();
        let prefab = prefab_storage.get(&prefab).unwrap();
        log::info!(
            "Set up an IRC row with {} entities from prefab: {:?}",
            num_descendants,
            prefab.entities().collect::<Vec<_>>()
        );
    }
}
