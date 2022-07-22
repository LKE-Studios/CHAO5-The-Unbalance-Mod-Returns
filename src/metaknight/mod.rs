mod game;
mod effect;
mod sound;
mod frame;
mod status;

pub fn install() {
    game::install();
    effect::install();
    sound::install();
    frame::install();
}