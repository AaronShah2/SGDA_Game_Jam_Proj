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

const IRC_DIALOG: &[(&str, &str, &str, [f32; 4])] = &[
    ("14:32:13", "Avaritia", "gg", [0.0, 0.676, 0.0, 1.0]),
    (
        "14:32:13",
        "Avaritia",
        "always fun to play against you",
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
    entities: Vec<Entity>,
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
            if self.dialog_number < IRC_DIALOG.len() {
                self.add_irc_dialog(&mut data);
            }
            self.dialog_number += 1;
        }
        data.data.update(&data.world);
        self.update_text(&mut data);
        Trans::None
    }

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        data.data.update(&data.world);
        self.update_text(data);
        Trans::None
    }

    fn on_stop(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        self.deinit_entities(&mut data.world);
    }
}

impl CutsceneState {
    fn update_text(&mut self, data: &mut StateData<GameData>) {
        let uitext_storage = &mut data.world.write_storage::<UiText>();
        let mut transform_storage = data.world.write_storage::<UiTransform>();
        let parents = data.world.read_resource::<ParentHierarchy>();
        self.entities
            .iter()
            .zip(IRC_DIALOG.iter())
            .enumerate()
            .flat_map(|(i, (&e, line))| {
                if let Some(transform) = transform_storage.get_mut(e) {
                    transform.local_y = -(i as f32 + 2.0) * 40.0;
                }
                parents
                    .all_children_iter(e)
                    .map(move |entity| (entity, line))
            })
            .collect::<Vec<_>>()
            .into_iter()
            .for_each(|(entity, (time, speaker, line, author_color))| {
                match transform_storage.get(entity).map(|e| &e.id) {
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
                        text.color = *author_color;
                    }
                    Some(label) if label == MESSAGE_LABEL => {
                        let text = uitext_storage
                            .get_mut(entity)
                            .expect("Timestamp has no UiText");
                        text.text = line.to_string();
                    }
                    label => log::info!("Did nothing to entity {:?}", label),
                }
            });
    }

    /// Despawn all entities produced by this
    fn deinit_entities(&mut self, world: &mut World) {
        for entity in self.entities.drain(..) {
            utils::delete_hierarchy(world, entity);
        }
    }

    /// Display a new line of IRC
    fn add_irc_dialog(&mut self, data: &mut StateData<GameData>) {
        let prefab = data
            .world
            .read_resource::<UiPrefabRegistry>()
            .find(data.world, IRC_ROW_ID)
            .expect("Couldn't find prefab for IRC line");
        let new_line = data.world.create_entity().with(prefab).build();
        data.data.update(&data.world);
        self.entities.push(new_line);
    }
}
