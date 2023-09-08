mod game;
mod effect;
mod frame;
mod sound;
mod expression;

pub fn install() {
    game::install();
    effect::install();    
    frame::install();
    sound::install();
    expression::install();
}