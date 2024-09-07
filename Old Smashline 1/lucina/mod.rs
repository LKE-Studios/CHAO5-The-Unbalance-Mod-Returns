mod game;
mod frame;
mod sound;
mod effect;
mod expression;

pub fn install() {
    game::install();    
    frame::install();
    sound::install();
    effect::install();
    expression::install();
}