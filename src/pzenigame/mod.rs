mod game;
mod effect;
mod expression;
mod sound;
mod status;
//mod frame;

pub fn install() {
    game::install();    
    effect::install();
    expression::install();
    status::install();
    sound::install();
    //frame::install();
}