mod game;
pub mod status;
pub mod frame;
pub mod effect;
pub mod sound;
mod expression;

pub fn install() {
    game::install();    
    frame::install();
    status::install();
    effect::install();
    sound::install();
    expression::install();
}
