mod game;
mod sound;
mod effect;
mod expression;
pub mod frame;
pub mod status;

pub fn install() {
    game::install();
    sound::install();   
    effect::install(); 
    expression::install();
    frame::install();
    status::install();
}