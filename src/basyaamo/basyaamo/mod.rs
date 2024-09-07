mod game;
mod effect;
mod sound;
mod expression;
mod frame;
mod status;

pub fn install() {
    game::install();   
    effect::install();
    frame::install(); 
    sound::install();
    expression::install();
    status::install();
}