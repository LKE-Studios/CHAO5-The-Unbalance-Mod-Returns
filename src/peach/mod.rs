mod game;
mod effect;
mod frame;

pub fn install() {
    game::install();
    effect::install();
    frame::install();
}
