mod game;
mod frame;
mod sound;
mod effect;
mod expression;
mod status;

pub fn install() {
    game::install();    
    frame::install();
    sound::install();
    effect::install();
    expression::install();
    status::install();
}