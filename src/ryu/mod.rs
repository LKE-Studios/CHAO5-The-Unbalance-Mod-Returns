mod game;
pub mod sound;
pub mod effect;
mod status;
mod frame;

pub fn install() {
    game::install();
    effect::install();
    sound::install();
    status::install();
    frame::install();
}
