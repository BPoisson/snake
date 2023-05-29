use ggez::audio::{SoundSource, Source};
use ggez::Context;
use rand::Rng;

pub struct Sounds {
    music: Source,
    food_sounds: Vec<Source>
}

impl Sounds {
    pub fn new(ctx: &Context) -> Self {
        return Sounds {
            music: Source::new(ctx, "\\sounds\\music.mp3").unwrap(),
            food_sounds: Sounds::get_food_sounds(ctx)
        }
    }

    fn get_food_sounds(ctx: &Context) -> Vec<Source> {
        let food1: Source = Source::new(ctx, "\\sounds\\food1.mp3").unwrap();
        let food2: Source = Source::new(ctx, "\\sounds\\food2.mp3").unwrap();
        let food3: Source = Source::new(ctx, "\\sounds\\food3.mp3").unwrap();
        let food4: Source = Source::new(ctx, "\\sounds\\food4.mp3").unwrap();
        let food5: Source = Source::new(ctx, "\\sounds\\food5.mp3").unwrap();

        vec![food1, food2, food3, food4, food5]
    }

    pub fn play_music(&mut self, ctx: &Context) -> () {
        if !self.music.playing() {
            self.music.set_repeat(true);
            self.music.set_volume(0.5);
            self.music.play(ctx).unwrap();
        }
    }

    pub fn stop_music(&mut self, ctx: &Context) -> () {
        if self.music.playing() {
            self.music.stop(ctx).unwrap();
        }
    }

    pub fn play_food_sound(&mut self, ctx: &Context) -> () {
        let sound_index: usize = rand::thread_rng().gen_range(0..self.food_sounds.len());

        if let Some(food_sound) = self.food_sounds.get_mut(sound_index) {
            food_sound.play_detached(ctx).unwrap();
        }
    }
}

