mod game;
//mod status;
pub mod sound;
pub mod effect;
mod status;

pub fn install() {
    game::install();
    effect::install();
    sound::install();
    status::install();
}
