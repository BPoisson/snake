use ggez::audio::Source;
use ggez::Context;

pub fn initialize_audio(ctx: &Context) -> Vec<Source> {
    let food1: Source = Source::new(ctx, "\\sounds\\food1.mp3").unwrap();
    let food2: Source = Source::new(ctx, "\\sounds\\food2.mp3").unwrap();
    let food3: Source = Source::new(ctx, "\\sounds\\food3.mp3").unwrap();
    let food4: Source = Source::new(ctx, "\\sounds\\food4.mp3").unwrap();
    let food5: Source = Source::new(ctx, "\\sounds\\food5.mp3").unwrap();

    vec![food1, food2, food3, food4, food5]
}