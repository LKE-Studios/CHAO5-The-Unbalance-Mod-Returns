mod game;
mod effect;
mod sound;
mod frame;
mod expression;

pub fn install() {
    game::install();
    effect::install();
    sound::install();
    frame::install();
    expression::install();
}
