use amethyst::{
    assets::{AssetStorage, Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    ecs::prelude::{World, WorldExt},
    ui::{UiLoader, UiPrefab},
    utils::application_root_dir,
};

#[derive(Default)]
pub struct UiPrefabRegistry {
    prefabs: Vec<Handle<UiPrefab>>,
}
impl UiPrefabRegistry {
    pub fn find(&self, world: &World, name: &str) -> Option<Handle<UiPrefab>> {
        let storage = world.read_resource::<AssetStorage<UiPrefab>>();
        self.prefabs.iter().find_map(|handle| {
            if storage.get(handle)?.entities().next()?.data()?.0.as_ref()?.id == name {
                Some(handle.clone())
            } else {
                None
            }
        })
    }

    pub fn print_prefabs(&self, world: &World) {
        let storage = world.read_resource::<AssetStorage<UiPrefab>>();
        self.prefabs
            .iter()
            .filter_map(|handle| Some(&storage.get(handle)?.entities().next()?.data()?.0.as_ref()?.id))
            .for_each(|name| print!("{} ", name));
        println!();
    }
}

pub fn initialize_prefabs(world: &mut World) -> ProgressCounter {
    use std::fs::read_dir;

    let mut counter = ProgressCounter::new();
    // Load UI Prefabs
    {
        let mut reg = UiPrefabRegistry::default();
        let prefab_path = application_root_dir()
            .unwrap()
            .join("assets")
            .join("prefabs")
            .join("ui");
        let prefab_iter = read_dir(prefab_path.to_str().unwrap()).unwrap();
        reg.prefabs = prefab_iter
            .filter_map(|entry| {
                if let Ok(file) = entry {
                    let file = file.path();
                    let filename = file.to_str()?;
                    if file.extension().map_or(false, |s| s.to_str() == Some("ron")) {
                        Some(world.exec(|loader: UiLoader<'_>| {
                            loader.load(
                                filename,
                                &mut counter,
                            )
                        }))
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
