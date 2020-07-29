use amethyst::{
    assets::{AssetStorage, Handle, Prefab, PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    derive::PrefabData,
    ecs::{Entity, World, WorldExt},
    ui::{UiLoader, UiPrefab},
    utils::application_root_dir,
    Error,
};
use crate::components::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default)]
pub struct UiPrefabRegistry {
    prefabs: Vec<Handle<UiPrefab>>,
}
impl super::ResourceRegistry for UiPrefabRegistry {
    type ResourceType = Handle<UiPrefab>;

    fn find(&self, world: &World, name: &str) -> Option<Self::ResourceType> {
        let storage = world.read_resource::<AssetStorage<UiPrefab>>();
        self.prefabs.iter().find_map(|handle| {
            if storage
                .get(handle)?
                .entities()
                .next()?
                .data()?
                .0
                .as_ref()?
                .id
                == name
            {
                Some(handle.clone())
            } else {
                None
            }
        })
    }
}

#[derive(Clone, Debug, Deserialize, PrefabData, Serialize)]
pub struct CharacterPrefab {
    player: Option<Player>,
    enemy: Option<Enemy>,
}

#[derive(Default)]
pub struct CharacterPrefabRegistry {
    prefabs: HashMap<String, Handle<Prefab<CharacterPrefab>>>,
}
impl super::ResourceRegistry for CharacterPrefabRegistry {
    type ResourceType = Handle<Prefab<CharacterPrefab>>;

    fn find(&self, _: &World, name: &str) -> Option<Self::ResourceType> {
        self.prefabs.get(name).cloned()
    }
}

pub fn initialize_prefabs(world: &mut World) -> ProgressCounter {
    let mut counter = ProgressCounter::new();
    // Load UI Prefabs
    {
        let mut reg = UiPrefabRegistry::default();
        let prefab_path = application_root_dir()
            .unwrap()
            .join("assets")
            .join("prefabs")
            .join("ui");
        let prefab_iter = std::fs::read_dir(prefab_path.to_str().unwrap()).unwrap();
        reg.prefabs = prefab_iter
            .filter_map(|entry| {
                if let Ok(file) = entry {
                    let file = file.path();
                    let filename = file.to_str()?;
                    if file
                        .extension()
                        .map_or(false, |s| s.to_str() == Some("ron"))
                    {
                        Some(world.exec(|loader: UiLoader<'_>| loader.load(filename, &mut counter)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        world.insert(reg);
    }
    // Load Character Prefabs
    {
        let mut reg = CharacterPrefabRegistry::default();
        let prefab_path = application_root_dir()
            .unwrap()
            .join("assets")
            .join("prefabs")
            .join("character");
        let prefab_iter = std::fs::read_dir(prefab_path.to_str().unwrap()).unwrap();
        reg.prefabs = prefab_iter
            .filter_map(|entry| {
                if let Ok(file) = entry {
                    let file = file.path();
                    let filename = file.to_str()?;
                    let filestem = file.file_stem()?.to_str()?.to_string();
                    if file
                        .extension()
                        .map_or(false, |s| s.to_str() == Some("ron"))
                    {
                        Some((filestem, world.exec(|loader: PrefabLoader<'_, CharacterPrefab>| loader.load(filename, RonFormat, &mut counter))))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        world.insert(reg);
    }
    counter
}
