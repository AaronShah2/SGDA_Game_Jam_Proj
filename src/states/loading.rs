use crate::{
    resources::{prefabs::initialize_prefabs, sprites::initialize_sprite_sheets},
    states::MenuState,
};

use amethyst::{assets::ProgressCounter, prelude::*};

#[derive(Default)]
pub struct LoadingState {
    counters: Vec<ProgressCounter>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.counters.push(initialize_prefabs(&mut data.world));
        self.counters
            .push(initialize_sprite_sheets(&mut data.world));
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        let loading: usize = self.counters.iter().map(ProgressCounter::num_loading).sum();
        let failed: usize = self.counters.iter().map(ProgressCounter::num_failed).sum();
        let finished: usize = self
            .counters
            .iter()
            .map(ProgressCounter::num_finished)
            .sum();
        println!(
            "Loading: {}, Failed: {}, Finished: {}",
            loading, failed, finished,
        );
        if loading == 0 {
            if failed != 0 {
                panic!(
                    "Failed loading assets :(\n\nErrors:\n{:#?}",
                    self.counters
                        .iter()
                        .map(|pc| pc.errors().into_iter())
                        .flatten()
                        .collect::<Vec<_>>()
                );
            }
            self.counters.clear();
            Trans::Switch(Box::new(MenuState::default()))
        } else {
            Trans::None
        }
    }
}
