mod game;
mod effect;
mod sound;
mod frame;

pub fn install() {
    game::install();
    effect::install();
    sound::install();
    frame::install();
}