use crate::{components::Player, resources::{HighScore, GameplayScoreDisplay}};
use amethyst::{
    core::transform::Transform,
    ecs::{Join, Read, ReadStorage, System, Write, WriteStorage},
    ui::UiText,
};

#[derive(Default)]
pub struct ScoreTrackingSystem;

impl<'s> System<'s> for ScoreTrackingSystem {
    type SystemData = (ReadStorage<'s, Player>, ReadStorage<'s, Transform>, Write<'s, HighScore>, Read<'s, GameplayScoreDisplay>, WriteStorage<'s, UiText>);

    fn run(&mut self, (players, transforms, mut high_score, score_displays, mut uitext): Self::SystemData) {
        high_score.max((&players, &transforms).join().map(|(_,t)| t.translation().y).max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap_or(0.0));
        let score = format!("Distance: {:.2} m", high_score.get_score());
        for &display in &score_displays.displays {
            if let Some(ref mut text) = uitext.get_mut(display) {
                text.text = score.clone();
            }
        }
    }
}
