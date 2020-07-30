use crate::{
    components::{Enemy, Player},
    resources::Paused,
};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

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
                    transform.prepend_translation(movement.normalize() * enemy.speed);
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
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Enemy>,
        Read<'s, Paused>,
    );

    fn run(&mut self, (transforms, players, enemies, paused): Self::SystemData) {
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
            }
        }
    }
}
