mod game;
mod effect;
mod frame;
mod sound;
mod status;
mod expression;

pub fn install() {
    game::install();
    effect::install();    
    frame::install();
    sound::install();
    status::install();
    expression::install();
}