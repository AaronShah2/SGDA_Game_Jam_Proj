use crate::{
    components::{Enemy, Player, Mud, Dog},
    resources::{CollisionEvent, Paused},
};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, Write, WriteStorage},
};
use shrev::EventChannel;

#[derive(SystemDesc)]
pub struct EnemyMovementSystem;

impl<'s> System<'s> for EnemyMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Enemy>,
        Read<'s, Paused>,
    );

    fn run(&mut self, (mut transforms, players, enemies, paused): Self::SystemData) {
        if *paused == Paused::Paused {
            return;
        }
        if let Some(player_position) = (&players, &transforms)
            .join()
            .next()
            .map(|(_, t)| *t.translation())
        {
            for (enemy, transform) in (&enemies, &mut transforms).join() {
                let movement = player_position - transform.translation();
                if movement.norm_squared() != 0.0 {
                    transform.prepend_translation(
                        movement.normalize()
                            * enemy.speed((player_position - transform.translation()).norm()),
                    );
                }
            }
        } else if (&enemies,).join().next().is_some() {
            log::warn!("No players found to pursue");
        }
    }
}

const COLLISION_RADIUS: f32 = 9.0;
#[derive(SystemDesc)]
pub struct EnemyCollisionSystem;

impl<'s> System<'s> for EnemyCollisionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Enemy>,
        Read<'s, Paused>,
        Write<'s, EventChannel<CollisionEvent>>,
    );

    fn run(
        &mut self,
        (transforms, players, enemies, paused, mut collision_channel): Self::SystemData,
    ) {
        if *paused == Paused::Paused {
            return;
        }
        for ((_, player_transform), (_, enemy_transform)) in
            (&players, &transforms).join().flat_map(|p| {
                (&enemies, &transforms)
                    .join()
                    .map(|e| (p, e))
                    .collect::<Vec<_>>()
            })
        {
            if (player_transform.translation() - enemy_transform.translation()).norm()
                <= COLLISION_RADIUS
            {
                log::info!("Collision between player and enemy");
                collision_channel.single_write(CollisionEvent);
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct EnemyObjectCollisionSystem;

impl<'s> System<'s> for EnemyObjectCollisionSystem {
    type SystemData = (
        WriteStorage<'s, Enemy>,
        ReadStorage<'s, Dog>,
        ReadStorage<'s, Mud>,
        Read<'s, Paused>,
    );

    fn run(&mut self, (mut enemies, dogs, muds, paused): Self::SystemData) {
        if *paused == Paused::Paused {
            return;
        }
        for (enemy,) in (&mut enemies,).join() {
            // checks if enemy is hit by dog
            let mut hit_by_dog: bool = false;
            for (dog,) in (&dogs,).join() {
                if dog.is_enemy_touching {
                    hit_by_dog = true;
                }
            }

            // checks if enemy is hit by mud
            let mut hit_by_mud: bool = false;
            for (mud,) in (&muds,).join() {
                if mud.is_enemy_touching {
                    hit_by_mud = true;
                }
            }

            // adjust enemy's speed bacsed on their collisions
            if hit_by_dog {
                enemy.stop();
            } else if hit_by_mud {
                enemy.slow_down();
            }
            else {
                enemy.normal_speed();
            }
        }
    }
}
