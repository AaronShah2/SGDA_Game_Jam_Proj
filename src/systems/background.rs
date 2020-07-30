use crate::components::{Background, Player};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Entities, Join, ReadStorage, System, SystemData},
};

const BACKGROUND_HEIGHT: f32 = 1440.0;

#[derive(SystemDesc)]
pub struct BackgroundRepeatSystem;

impl<'s> System<'s> for BackgroundRepeatSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Background>,
        Entities<'s>,
    );

    fn run(&mut self, (transforms, players, backgrounds, entities): Self::SystemData) {
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
                    > 2.0 * BACKGROUND_HEIGHT
                {
                    entities
                        .delete(background)
                        .expect("Error removing background");
                }
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
