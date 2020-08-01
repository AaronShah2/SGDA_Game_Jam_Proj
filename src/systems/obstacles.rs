use crate::{
    components::{Dog, Mud, Car, Player},
    resources::{
        prefabs::{ObstaclePrefab, ObstaclePrefabRegistry},
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
use rand::prelude::*;

const BACKGROUND_HEIGHT: f32 = 1440.0;
const BACKGROUND_WIDTH: f32 = 1500.0;

const OBSTACLE_CREATION_HEIGHT: f32 = 1000.0;

#[derive(SystemDesc)]
pub struct ObstacleRandomizationSystem;

impl<'s> System<'s> for ObstacleRandomizationSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Mud>,
        ReadStorage<'s, Car>,
        ReadStorage<'s, Dog>,
        Read<'s, ObstaclePrefabRegistry>,
        WriteStorage<'s, Handle<Prefab<ObstaclePrefab>>>,
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
            muds,
            cars,
            dogs,
            prefab_registry,
            mut obstacle_prefab_handles,
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
            if player_position.y > OBSTACLE_CREATION_HEIGHT {
                self.clear_distant_mud(&entities, &muds, &transforms, &player_position);
                self.clear_distant_car(&entities, &cars, &transforms, &player_position);
                self.clear_distant_dog(&entities, &dogs, &transforms, &player_position);
                self.place_needed_top_mud(
                    &entities,
                    &muds,
                    &mut transforms,
                    &player_position,
                    &prefab_registry,
                    &mut obstacle_prefab_handles,
                    &spritesheet_registry,
                    &spritesheet_storage,
                    &mut sprite_render_storage,
                );
                self.place_needed_top_car(
                    &entities,
                    &cars,
                    &mut transforms,
                    &player_position,
                    &prefab_registry,
                    &mut obstacle_prefab_handles,
                    &spritesheet_registry,
                    &spritesheet_storage,
                    &mut sprite_render_storage,
                );
                self.place_needed_top_dog(
                    &entities,
                    &dogs,
                    &mut transforms,
                    &player_position,
                    &prefab_registry,
                    &mut obstacle_prefab_handles,
                    &spritesheet_registry,
                    &spritesheet_storage,
                    &mut sprite_render_storage,
                );
            }
        } else {
            // The player is gone, so remove all obstacles
            for (mud, _) in (&entities, &muds).join() {
                entities
                    .delete(mud)
                    .expect("Error removing mud");
            }
            for (car, _) in (&entities, &cars).join() {
                entities
                    .delete(car)
                    .expect("Error removing car");
            }
            for (dog, _) in (&entities, &dogs).join() {
                entities
                    .delete(dog)
                    .expect("Error removing dog");
            }
        }
    }
}

impl ObstacleRandomizationSystem {
    /// Clear obstacles which are more than 4 obstacle heights away
    /// from the player, to avoid having too many obstacles live simultaneously
    fn clear_distant_mud<'s>(
        &self,
        entities: &Entities<'s>,
        muds: &ReadStorage<'s, Mud>,
        transforms: &WriteStorage<'s, Transform>,
        player_position: &Vector3<f32>,
    ) {
        for (mud, _, mud_position) in (entities, muds, transforms).join() {
            if (player_position - mud_position.translation()).norm()
                > 4.0 * BACKGROUND_HEIGHT
            {
                entities
                    .delete(mud)
                    .expect("Error removing mud");
            }
        }
    }

    fn clear_distant_car<'s>(
        &self,
        entities: &Entities<'s>,
        cars: &ReadStorage<'s, Car>,
        transforms: &WriteStorage<'s, Transform>,
        player_position: &Vector3<f32>,
    ) {
        for (car, _, car_position) in (entities, cars, transforms).join() {
            if (player_position - car_position.translation()).norm()
                > 4.0 * BACKGROUND_HEIGHT
            {
                entities
                    .delete(car)
                    .expect("Error removing car");
            }
        }
    }

    fn clear_distant_dog<'s>(
        &self,
        entities: &Entities<'s>,
        dogs: &ReadStorage<'s, Dog>,
        transforms: &WriteStorage<'s, Transform>,
        player_position: &Vector3<f32>,
    ) {
        for (dog, _, dog_position) in (entities, dogs, transforms).join() {
            if (player_position - dog_position.translation()).norm()
                > 4.0 * BACKGROUND_HEIGHT
            {
                entities
                    .delete(dog)
                    .expect("Error removing dog");
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    /// Place a new mud down if the player is too close to the top
    fn place_needed_top_mud<'s>(
        &self,
        entities: &Entities<'s>,
        muds: &ReadStorage<'s, Mud>,
        transforms: &mut WriteStorage<'s, Transform>,
        player_position: &Vector3<f32>,
        prefab_registry: &Read<'s, ObstaclePrefabRegistry>,
        obstacle_prefab_handles: &mut WriteStorage<'s, Handle<Prefab<ObstaclePrefab>>>,
        spritesheet_registry: &Read<'s, SpriteSheetRegister>,
        spritesheet_storage: &Read<'s, AssetStorage<SpriteSheet>>,
        sprite_render_storage: &mut WriteStorage<'s, SpriteRender>,
    ) {
        let max_height = (muds, transforms as &WriteStorage<'s, Transform>)
            .join()
            .map(|(_, t)| t.translation().y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or_else(|| {
                player_position.y - player_position.y % BACKGROUND_HEIGHT - BACKGROUND_HEIGHT
            });
        if max_height - player_position.y < BACKGROUND_HEIGHT {
            let mut rng = rand::thread_rng();

            // randomly generates y coordinate
            let y_rng:f32 = rng.gen();
            let y_pos = max_height + (y_rng * BACKGROUND_HEIGHT);

            // randomly generates x coordinates
            let x_rng:f32 = rng.gen();
            let x_pos:f32 = (x_rng * BACKGROUND_WIDTH) - (BACKGROUND_WIDTH/2.0);

            let sprite_render = spritesheet_registry
                .find_sprite_sans_world(&*spritesheet_storage, "mud", 0)
                .unwrap_or_else(|| panic!("Couldn't find spritesheet mud"));
            let obstacle_prefab = prefab_registry
                .find_sans_world("mud")
                .expect("Couldn't find obstacle prefab");
            let new_section = entities
                .build_entity()
                .with(obstacle_prefab, obstacle_prefab_handles)
                .with(sprite_render, sprite_render_storage)
                .build();
            transforms
                .get_mut_or_default(new_section)
                .map(|transform| {
                    transform.translation_mut().x = x_pos;
                    transform.translation_mut().y = y_pos;
                    *transform.scale_mut() *= 40.0;
                })
                .unwrap_or_else(|| panic!("Couldn't update the translation"));
        }
    }

    #[allow(clippy::too_many_arguments)]
    /// Place a new car down if the player is too close to the top
    fn place_needed_top_car<'s>(
        &self,
        entities: &Entities<'s>,
        cars: &ReadStorage<'s, Car>,
        transforms: &mut WriteStorage<'s, Transform>,
        player_position: &Vector3<f32>,
        prefab_registry: &Read<'s, ObstaclePrefabRegistry>,
        obstacle_prefab_handles: &mut WriteStorage<'s, Handle<Prefab<ObstaclePrefab>>>,
        spritesheet_registry: &Read<'s, SpriteSheetRegister>,
        spritesheet_storage: &Read<'s, AssetStorage<SpriteSheet>>,
        sprite_render_storage: &mut WriteStorage<'s, SpriteRender>,
    ) {
        let max_height = (cars, transforms as &WriteStorage<'s, Transform>)
            .join()
            .map(|(_, t)| t.translation().y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or_else(|| {
                player_position.y - player_position.y % BACKGROUND_HEIGHT - BACKGROUND_HEIGHT
            });
        if max_height - player_position.y < BACKGROUND_HEIGHT {
            let mut rng = rand::thread_rng();

            // randomly generates y coordinate
            let y_rng:f32 = rng.gen();
            let y_pos = max_height + (y_rng * BACKGROUND_HEIGHT);

            // randomly generates x coordinates
            let x_rng:f32 = rng.gen();
            let x_pos:f32 = (x_rng * BACKGROUND_WIDTH) - (BACKGROUND_WIDTH/2.0);

            let sprite_render = spritesheet_registry
                .find_sprite_sans_world(&*spritesheet_storage, "car", 0)
                .unwrap_or_else(|| panic!("Couldn't find spritesheet car"));
            let obstacle_prefab = prefab_registry
                .find_sans_world("car")
                .expect("Couldn't find obstacle prefab");
            let new_section = entities
                .build_entity()
                .with(obstacle_prefab, obstacle_prefab_handles)
                .with(sprite_render, sprite_render_storage)
                .build();
            transforms
                .get_mut_or_default(new_section)
                .map(|transform| {
                    transform.translation_mut().x = x_pos;
                    transform.translation_mut().y = y_pos;
                    *transform.scale_mut() *= 40.0;
                })
                .unwrap_or_else(|| panic!("Couldn't update the translation"));
        }
    }

    #[allow(clippy::too_many_arguments)]
    /// Place a new dog down if the player is too close to the top
    fn place_needed_top_dog<'s>(
        &self,
        entities: &Entities<'s>,
        dogs: &ReadStorage<'s, Dog>,
        transforms: &mut WriteStorage<'s, Transform>,
        player_position: &Vector3<f32>,
        prefab_registry: &Read<'s, ObstaclePrefabRegistry>,
        obstacle_prefab_handles: &mut WriteStorage<'s, Handle<Prefab<ObstaclePrefab>>>,
        spritesheet_registry: &Read<'s, SpriteSheetRegister>,
        spritesheet_storage: &Read<'s, AssetStorage<SpriteSheet>>,
        sprite_render_storage: &mut WriteStorage<'s, SpriteRender>,
    ) {
        let max_height = (dogs, transforms as &WriteStorage<'s, Transform>)
            .join()
            .map(|(_, t)| t.translation().y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or_else(|| {
                player_position.y - player_position.y % BACKGROUND_HEIGHT - BACKGROUND_HEIGHT
            });
        if max_height - player_position.y < BACKGROUND_HEIGHT {
            let mut rng = rand::thread_rng();

            // randomly generates y coordinate
            let y_rng:f32 = rng.gen();
            let y_pos = max_height + (y_rng * BACKGROUND_HEIGHT);

            // randomly generates x coordinates
            let x_rng:f32 = rng.gen();
            let x_pos:f32 = max_height + (x_rng * BACKGROUND_WIDTH) 
            - (BACKGROUND_WIDTH/2.0);

            let sprite_render = spritesheet_registry
                .find_sprite_sans_world(&*spritesheet_storage, "dog", 0)
                .unwrap_or_else(|| panic!("Couldn't find spritesheet dog"));
            let obstacle_prefab = prefab_registry
                .find_sans_world("dog")
                .expect("Couldn't find obstacle prefab");
            let new_section = entities
                .build_entity()
                .with(obstacle_prefab, obstacle_prefab_handles)
                .with(sprite_render, sprite_render_storage)
                .build();
            transforms
                .get_mut_or_default(new_section)
                .map(|transform| {
                    transform.translation_mut().x = x_pos;
                    transform.translation_mut().y = y_pos;
                    *transform.scale_mut() *= 10.0;
                })
                .unwrap_or_else(|| panic!("Couldn't update the translation"));
        }
    }
}