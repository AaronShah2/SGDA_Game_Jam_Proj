use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, ReadStorage, System, SystemData, WriteStorage},
};
use crate::components::{Enemy, Player};

const MOVE_SPEED: f32 = 9.0f32;

#[derive(SystemDesc)]
pub struct EnemyMovementSystem;

impl<'s> System<'s> for EnemyMovementSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Enemy>,
    );

    fn run(&mut self, (mut transforms, players, enemies): Self::SystemData) {
        if let Some(player_position) = (&players, &transforms).join().next().map(|(_, t)| t.translation().clone()) {
            for (_, transform) in (&enemies, &mut transforms).join() {
                let movement = player_position - transform.translation();
                if movement.norm_squared() != 0.0 {
                    transform.prepend_translation(movement.normalize() * MOVE_SPEED);
                }
            }
        } else {
            if (&enemies,).join().next().is_some() {
                log::warn!("No players found to pursue");
            }
        }
    }
}
