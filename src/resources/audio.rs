use amethyst::{
    assets::{Loader, ProgressCounter},
    audio::{output::init_output, AudioSink, SourceHandle, WavFormat},
    prelude::*,
};

use std::{iter::Cycle, vec::IntoIter};

const SOUNDTRACK: &'static [&'static str] = &["audio/BeepBox-Song.wav"];

pub struct Music {
    pub music: Cycle<IntoIter<SourceHandle>>,
}

fn load_audio_song(
    loader: &Loader,
    world: &World,
    file: &str,
    counter: &mut ProgressCounter,
) -> SourceHandle {
    loader.load(file, WavFormat, counter, &world.read_resource())
}

pub fn initialize_audio(world: &mut World) -> ProgressCounter {
    let mut counter = ProgressCounter::new();
    init_output(world);
    let music = {
        let loader = world.read_resource::<Loader>();
        let mut sink = world.write_resource::<AudioSink>();
        sink.set_volume(1.0);
        let music = SOUNDTRACK
            .iter()
            .map(|file| load_audio_song(&loader, world, file, &mut counter))
            .collect::<Vec<_>>()
            .into_iter()
            .cycle();
        Music { music }
    };
    world.insert(music);
    counter
}
