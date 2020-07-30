use amethyst::{
    assets::{AssetStorage, Handle, Loader, ProgressCounter},
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    utils::application_root_dir,
};
use log::warn;
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
impl SpriteSheetRegister {
    pub fn find_sprite(&self, world: &World, name: &str, index: usize) -> Option<SpriteRender> {
        self.find_sprite_sans_world(&*world.read_resource::<AssetStorage<SpriteSheet>>(), name, index)
    }
    pub fn find_sprite_sans_world(&self, sprite_sheet_storage: &AssetStorage<SpriteSheet>, name: &str, index: usize) -> Option<SpriteRender> {
        self.sprite_sheets.get(name).map_or_else(
            || {
                warn!(
                    "Tried to load sprite #{} from non-existant sheet {}.\nExisting sheets: {}",
                    index,
                    name,
                    self.sprite_sheets
                        .keys()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(", ")
                );
                None
            },
            |sprite_sheet| {
                let sheet_length = sprite_sheet_storage
                    .get(&sprite_sheet)?
                    .sprites
                    .len();
                if index >= sheet_length {
                    warn!(
                        "Tried to load sprite #{}/{} on sheet {}",
                        index, sheet_length, name
                    );
                    None
                } else {
                    Some(SpriteRender {
                        sprite_sheet: sprite_sheet.clone(),
                        sprite_number: index,
                    })
                }
            },
        )
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
                std::fs::metadata(format!("assets/sprites/{}.ron", filename)).ok()?;
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
