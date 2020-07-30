use crate::{
    components::{Background, Player},
    resources::{prefabs::{BackgroundPrefab, BackgroundPrefabRegistry}, sprites::SpriteSheetRegister},
};
use amethyst::{
    assets::{AssetStorage, Handle, Prefab},
    core::Transform,
    derive::SystemDesc,
    ecs::{storage::GenericWriteStorage, Entities, Join, Read, ReadStorage, System, SystemData, WriteStorage},
    renderer::{SpriteRender, SpriteSheet},
};

const BACKGROUND_HEIGHT: f32 = 1440.0;

#[derive(SystemDesc)]
pub struct BackgroundRepeatSystem;

impl<'s> System<'s> for BackgroundRepeatSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Background>,
        Read<'s, BackgroundPrefabRegistry>,
        WriteStorage<'s, Handle<Prefab<BackgroundPrefab>>>,
        Read<'s, SpriteSheetRegister>,
        Read<'s, AssetStorage<SpriteSheet>>,
        WriteStorage<'s, SpriteRender>,
        Entities<'s>,
    );

    fn run(&mut self, (mut transforms, players, backgrounds, prefab_registry, mut background_prefab_handles, spritesheet_registry, spritesheet_storage, mut sprite_render_storage, entities): Self::SystemData) {
        if let Some(player_position) = (&players, &transforms)
            .join()
            .next()
            .map(|(_, t)| *t.translation())
        {
            // Clear backgrounds which are more than 2 background heights away from the player
            for (background, _, background_position) in
                (&entities, &backgrounds, &transforms).join()
            {
                if (player_position - background_position.translation()).norm()
                    > 4.0 * BACKGROUND_HEIGHT
                {
                    entities
                        .delete(background)
                        .expect("Error removing background");
                }
            }
            // Place a new background down if the player is too close to the top
            let max_height = (&backgrounds, &transforms)
                .join()
                .map(|(_, t)| t.translation().y)
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap_or_else(|| player_position.y - player_position.y % BACKGROUND_HEIGHT - BACKGROUND_HEIGHT);
            if max_height - player_position.y < BACKGROUND_HEIGHT {
                let y_pos = max_height + BACKGROUND_HEIGHT;
                log::info!("Placing another background section at {}, for a total of {} backgrounds", y_pos, (&backgrounds,).join().count());
                let sprite_render = spritesheet_registry
                    .find_sprite_sans_world(&*spritesheet_storage, "BG", 0)
                    .unwrap_or_else(|| panic!("Couldn't find spritesheet BG"));
                let background_prefab = prefab_registry
                    .find_sans_world("background")
                    .expect("Couldn't find background prefab");
                let new_section = entities.build_entity()
                    .with(background_prefab, &mut background_prefab_handles)
                    .with(sprite_render, &mut sprite_render_storage)
                    .build();
                transforms.get_mut_or_default(new_section).map(|transform| {
                    transform.translation_mut().y = y_pos;
                    *transform.scale_mut() *= 3.0;
                })
                    .unwrap_or_else(|| panic!("Couldn't update the translation"));
            }
        } else {
            // The player is gone, so remove all backgrounds
            for (background, _) in (&entities, &backgrounds).join() {
                entities
                    .delete(background)
                    .expect("Error removing background");
            }
        }
    }
}
