mod game;
mod frame;
mod effect;
mod sound;
mod expression;
mod status;

pub fn install() {
    game::install();   
    frame::install(); 
    effect::install();
    sound::install();
    expression::install();
    status::install();
}