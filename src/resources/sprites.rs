pub use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::*,
    renderer::{ImageFormat, SpriteSheet, SpriteSheetFormat, Texture},
    utils::application_root_dir,
};
use std::collections::HashMap;

#[derive(Default)]
pub struct SpriteSheetRegister {
    sprite_sheets: HashMap<String, Handle<SpriteSheet>>,
}
impl super::ResourceRegistry for SpriteSheetRegister {
    type ResourceType = Handle<SpriteSheet>;

    fn find(&self, _: &World, name: &str) -> Option<Self::ResourceType> {
        self.sprite_sheets.get(name).cloned()
    }
}

pub fn initialize_sprite_sheets(world: &mut World) -> ProgressCounter {
    let mut counter = ProgressCounter::new();
    let mut reg = SpriteSheetRegister::default();
    let sprite_sheet_path = application_root_dir()
        .unwrap()
        .join("assets")
        .join("sprites");
    let sprite_sheet_iter = std::fs::read_dir(sprite_sheet_path.to_str().unwrap()).unwrap();
    reg.sprite_sheets = sprite_sheet_iter
        .filter_map(|entry| {
            let file = entry.ok()?;
            let file = file.path();
            let filename = file.file_stem()?.to_str()?;
            let extension = file.extension()?.to_str()?;
            if extension != "png" && extension != "jpeg" && extension != "jpg" {
                return None;
            }
            Some((filename.to_string(), {
                let loader = world.read_resource::<Loader>();
                let texture_storage = world.read_resource::<AssetStorage<Texture>>();
                let texture_handle = loader.load(
                    format!("sprites/{}.{}", filename, extension),
                    ImageFormat::default(),
                    &mut counter,
                    &texture_storage,
                );
                let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
                loader.load(
                    format!("sprites/{}.ron", filename),
                    SpriteSheetFormat(texture_handle),
                    &mut counter,
                    &sprite_sheet_store,
                )
            }))
        })
        .collect();
    world.insert(reg);
    counter
}
