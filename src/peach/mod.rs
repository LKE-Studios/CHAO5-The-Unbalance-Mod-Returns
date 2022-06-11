mod game;
pub mod status;
mod effect;
mod sound;

pub fn install() {
    game::install();
    effect::install();
    status::install();
    sound::install();
}
