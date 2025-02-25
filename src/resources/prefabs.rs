use crate::components::*;
use amethyst::{
    assets::{AssetStorage, Handle, Prefab, PrefabData, PrefabLoader, ProgressCounter, RonFormat},
    core::transform::Transform,
    derive::PrefabData,
    ecs::{Entity, World, WorldExt, WriteStorage},
    renderer::Camera,
    ui::{UiLoader, UiPrefab},
    utils::application_root_dir,
    Error,
};
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
        self.prefabs
            .iter()
            .find_map(|handle| {
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
            .or_else(|| {
                // Warn that the desired prefab doesn't exist
                log::warn!(
                    "Tried to load non-existing Ui Prefab {}.\n Existing prefabs are: {}",
                    name,
                    self.prefabs
                        .iter()
                        .filter_map(|handle| Some(
                            storage
                                .get(handle)?
                                .entities()
                                .next()?
                                .data()?
                                .0
                                .as_ref()?
                                .id
                                .to_string()
                        ))
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                None
            })
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CameraAdapterPrefab {
    width: f32,
    height: f32,
}
impl<'a> PrefabData<'a> for CameraAdapterPrefab {
    type SystemData = WriteStorage<'a, Camera>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        cameras: &mut Self::SystemData,
        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<Self::Result, Error> {
        cameras.insert(entity, Camera::standard_2d(self.width, self.height))?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct TransformAdapterPrefab {
    pos2d: Option<(f32, f32)>,
    pos3d: Option<(f32, f32, f32)>,
    scale: Option<f32>,
    layer: Option<f32>,
}

impl<'a> PrefabData<'a> for TransformAdapterPrefab {
    type SystemData = WriteStorage<'a, Transform>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        transforms: &mut Self::SystemData,
        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<Self::Result, Error> {
        let mut transform = Transform::default();
        if let Some((x, y)) = self.pos2d {
            // Handles layering
            if let Some(layer) = self.layer {
                transform.set_translation_xyz(x, y, layer);
            } else {
                transform.set_translation_xyz(x, y, 0.0);
            }
        }
        if let Some((x, y, z)) = self.pos3d {
            transform.set_translation_xyz(x, y, z);
        }
        if let Some(scale) = self.scale {
            *transform.scale_mut() *= scale;
        }
        transforms.insert(entity, transform)?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlayerPrefab;
impl<'a> PrefabData<'a> for PlayerPrefab {
    type SystemData = WriteStorage<'a, Player>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        players: &mut Self::SystemData,
        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<Self::Result, Error> {
        players.insert(entity, Player::default())?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub struct EnemyPrefab;
impl<'a> PrefabData<'a> for EnemyPrefab {
    type SystemData = WriteStorage<'a, Enemy>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        enemies: &mut Self::SystemData,
        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<Self::Result, Error> {
        enemies.insert(entity, Enemy::default())?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub struct CarPrefab;
impl<'a> PrefabData<'a> for CarPrefab {
    type SystemData = WriteStorage<'a, Car>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        enemies: &mut Self::SystemData,
        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<Self::Result, Error> {
        enemies.insert(entity, Car::default())?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub struct MudPrefab;
impl<'a> PrefabData<'a> for MudPrefab {
    type SystemData = WriteStorage<'a, Mud>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        enemies: &mut Self::SystemData,
        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<Self::Result, Error> {
        enemies.insert(entity, Mud::default())?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, Default, Deserialize, Serialize)]
pub struct DogPrefab;
impl<'a> PrefabData<'a> for DogPrefab {
    type SystemData = WriteStorage<'a, Dog>;
    type Result = ();

    fn add_to_entity(
        &self,
        entity: Entity,
        enemies: &mut Self::SystemData,
        _entities: &[Entity],
        _children: &[Entity],
    ) -> Result<Self::Result, Error> {
        enemies.insert(entity, Dog::default())?;
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, PrefabData, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CharacterPrefab {
    camera: Option<CameraAdapterPrefab>,
    enemy: Option<EnemyPrefab>,
    player: Option<PlayerPrefab>,
    position: Option<TransformAdapterPrefab>,
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

// prefabs for background
#[derive(Clone, Debug, Deserialize, PrefabData, Serialize)]
#[serde(deny_unknown_fields)]
pub struct BackgroundPrefab {
    position: Option<TransformAdapterPrefab>,
    background: Background,
}

#[derive(Default)]
pub struct BackgroundPrefabRegistry {
    prefabs: HashMap<String, Handle<Prefab<BackgroundPrefab>>>,
}

impl super::ResourceRegistry for BackgroundPrefabRegistry {
    type ResourceType = Handle<Prefab<BackgroundPrefab>>;

    fn find(&self, _: &World, name: &str) -> Option<Self::ResourceType> {
        self.prefabs.get(name).cloned()
    }
}
impl BackgroundPrefabRegistry {
    pub fn find_sans_world(
        &self,
        name: &str,
    ) -> Option<<Self as super::ResourceRegistry>::ResourceType> {
        self.prefabs.get(name).cloned()
    }
}

// prefabs for obstacles
#[derive(Clone, Debug, Deserialize, PrefabData, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ObstaclePrefab {
    mud: Option<MudPrefab>,
    mudposition: Option<TransformAdapterPrefab>,
    car: Option<CarPrefab>,
    carposition: Option<TransformAdapterPrefab>,
    dog: Option<DogPrefab>,
    dogposition: Option<TransformAdapterPrefab>,
}

#[derive(Default)]
pub struct ObstaclePrefabRegistry {
    prefabs: HashMap<String, Handle<Prefab<ObstaclePrefab>>>,
}

impl ObstaclePrefabRegistry {
    pub fn find_sans_world(
        &self,
        name: &str,
    ) -> Option<<Self as super::ResourceRegistry>::ResourceType> {
        self.prefabs.get(name).cloned()
    }
}

impl super::ResourceRegistry for ObstaclePrefabRegistry {
    type ResourceType = Handle<Prefab<ObstaclePrefab>>;

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
    // load background prefabs
    {
        let mut reg = BackgroundPrefabRegistry::default();
        let prefab_path = application_root_dir()
            .unwrap()
            .join("assets")
            .join("prefabs")
            .join("background");
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
                        Some((
                            filestem,
                            world.exec(|loader: PrefabLoader<'_, BackgroundPrefab>| {
                                loader.load(filename, RonFormat, &mut counter)
                            }),
                        ))
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();
        world.insert(reg);
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
                            Some((
                                filestem,
                                world.exec(|loader: PrefabLoader<'_, CharacterPrefab>| {
                                    loader.load(filename, RonFormat, &mut counter)
                                }),
                            ))
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

        // Load Obstacle Prefabs
        {
            let mut reg = ObstaclePrefabRegistry::default();
            let prefab_path = application_root_dir()
                .unwrap()
                .join("assets")
                .join("prefabs")
                .join("obstacle");
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
                            Some((
                                filestem,
                                world.exec(|loader: PrefabLoader<'_, ObstaclePrefab>| {
                                    loader.load(filename, RonFormat, &mut counter)
                                }),
                            ))
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
    }
    counter
}
