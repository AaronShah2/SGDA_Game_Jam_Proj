use crate::{
    components::{Background, Player},
    resources::{
        prefabs::{BackgroundPrefab, BackgroundPrefabRegistry},
        sprites::SpriteSheetRegister,
        Paused,
    },
};
use amethyst::{
    assets::{AssetStorage, Handle, Prefab},
    core::Transform,
    derive::SystemDesc,
    ecs::{
        storage::GenericWriteStorage, Entities, Join, Read, ReadStorage, System, SystemData,
        WriteStorage,
    },
    renderer::{SpriteRender, SpriteSheet},
};
use nalgebra::Vector3;

const BACKGROUND_HEIGHT: f32 = 1440.0;

#[derive(SystemDesc)]
pub struct BackgroundRepeatSystem;

impl<'s> System<'s> for BackgroundRepeatSystem {
    #[allow(clippy::type_complexity)]
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
        Read<'s, Paused>,
    );

    fn run(
        &mut self,
        (
            mut transforms,
            players,
            backgrounds,
            prefab_registry,
            mut background_prefab_handles,
            spritesheet_registry,
            spritesheet_storage,
            mut sprite_render_storage,
            entities,
            paused,
        ): Self::SystemData,
    ) {
        if *paused == Paused::Paused {
            return;
        }
        if let Some(player_position) = (&players, &transforms)
            .join()
            .next()
            .map(|(_, t)| *t.translation())
        {
            self.clear_distant_backgrounds(&entities, &backgrounds, &transforms, &player_position);
            self.place_needed_top_backgrounds(
                &entities,
                &backgrounds,
                &mut transforms,
                &player_position,
                &prefab_registry,
                &mut background_prefab_handles,
                &spritesheet_registry,
                &spritesheet_storage,
                &mut sprite_render_storage,
            );
            self.place_needed_bottom_backgrounds(
                &entities,
                &backgrounds,
                &mut transforms,
                &player_position,
                &prefab_registry,
                &mut background_prefab_handles,
                &spritesheet_registry,
                &spritesheet_storage,
                &mut sprite_render_storage,
            );
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
impl BackgroundRepeatSystem {
    /// Clear backgrounds which are more than 4 background heights away
    /// from the player, to avoid having too many backgrounds live simultaneously
    fn clear_distant_backgrounds<'s>(
        &self,
        entities: &Entities<'s>,
        backgrounds: &ReadStorage<'s, Background>,
        transforms: &WriteStorage<'s, Transform>,
        player_position: &Vector3<f32>,
    ) {
        for (background, _, background_position) in (entities, backgrounds, transforms).join() {
            if (player_position - background_position.translation()).norm()
                > 4.0 * BACKGROUND_HEIGHT
            {
                entities
                    .delete(background)
                    .expect("Error removing background");
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    /// Place a new background down if the player is too close to the top
    fn place_needed_top_backgrounds<'s>(
        &self,
        entities: &Entities<'s>,
        backgrounds: &ReadStorage<'s, Background>,
        transforms: &mut WriteStorage<'s, Transform>,
        player_position: &Vector3<f32>,
        prefab_registry: &Read<'s, BackgroundPrefabRegistry>,
        background_prefab_handles: &mut WriteStorage<'s, Handle<Prefab<BackgroundPrefab>>>,
        spritesheet_registry: &Read<'s, SpriteSheetRegister>,
        spritesheet_storage: &Read<'s, AssetStorage<SpriteSheet>>,
        sprite_render_storage: &mut WriteStorage<'s, SpriteRender>,
    ) {
        let max_height = (backgrounds, transforms as &WriteStorage<'s, Transform>)
            .join()
            .map(|(_, t)| t.translation().y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or_else(|| {
                player_position.y - player_position.y % BACKGROUND_HEIGHT - BACKGROUND_HEIGHT
            });
        if max_height - player_position.y < BACKGROUND_HEIGHT {
            let y_pos = max_height + BACKGROUND_HEIGHT;
            let sprite_render = spritesheet_registry
                .find_sprite_sans_world(&*spritesheet_storage, "BG", 0)
                .unwrap_or_else(|| panic!("Couldn't find spritesheet BG"));
            let background_prefab = prefab_registry
                .find_sans_world("background")
                .expect("Couldn't find background prefab");
            let new_section = entities
                .build_entity()
                .with(background_prefab, background_prefab_handles)
                .with(sprite_render, sprite_render_storage)
                .build();
            transforms
                .get_mut_or_default(new_section)
                .map(|transform| {
                    transform.translation_mut().y = y_pos;
                    *transform.scale_mut() *= 3.0;
                })
                .unwrap_or_else(|| panic!("Couldn't update the translation"));
        }
    }

    #[allow(clippy::too_many_arguments)]
    /// Place a new background down if the player is too close to the bottom
    fn place_needed_bottom_backgrounds<'s>(
        &self,
        entities: &Entities<'s>,
        backgrounds: &ReadStorage<'s, Background>,
        transforms: &mut WriteStorage<'s, Transform>,
        player_position: &Vector3<f32>,
        prefab_registry: &Read<'s, BackgroundPrefabRegistry>,
        background_prefab_handles: &mut WriteStorage<'s, Handle<Prefab<BackgroundPrefab>>>,
        spritesheet_registry: &Read<'s, SpriteSheetRegister>,
        spritesheet_storage: &Read<'s, AssetStorage<SpriteSheet>>,
        sprite_render_storage: &mut WriteStorage<'s, SpriteRender>,
    ) {
        let min_height = (backgrounds, transforms as &WriteStorage<'s, Transform>)
            .join()
            .map(|(_, t)| t.translation().y)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or_else(|| player_position.y - player_position.y % BACKGROUND_HEIGHT);
        if player_position.y - min_height < BACKGROUND_HEIGHT {
            let y_pos = min_height - BACKGROUND_HEIGHT;
            let sprite_render = spritesheet_registry
                .find_sprite_sans_world(&*spritesheet_storage, "BG", 0)
                .unwrap_or_else(|| panic!("Couldn't find spritesheet BG"));
            let background_prefab = prefab_registry
                .find_sans_world("background")
                .expect("Couldn't find background prefab");
            let new_section = entities
                .build_entity()
                .with(background_prefab, background_prefab_handles)
                .with(sprite_render, sprite_render_storage)
                .build();
            transforms
                .get_mut_or_default(new_section)
                .map(|transform| {
                    transform.translation_mut().y = y_pos;
                    *transform.scale_mut() *= 3.0;
                })
                .unwrap_or_else(|| panic!("Couldn't update the translation"));
        }
    }
}
