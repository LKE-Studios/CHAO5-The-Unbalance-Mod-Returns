mod game;
mod effect;
mod sound;
mod expression;
mod frame;
mod status;

pub fn install() {
    game::install();    
    effect::install();
    sound::install();
    expression::install();
    frame::install();
    status::install();
}