mod status;
mod game;
mod effect;
mod sound;

pub fn install() {
    status::install();
    game::install();
    effect::install();
    sound::install();
}