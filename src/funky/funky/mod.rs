mod game;
mod sound;
mod effect;
mod expression;
mod frame;
mod status;

pub fn install() {
    game::install();
    sound::install();
    effect::install();
    frame::install();
    expression::install();
    status::install();
}