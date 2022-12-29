mod game;
mod sound;
mod effect;
mod hook;
mod expression;
//mod frame;

pub fn install() {
    game::install(); 
    sound::install();  
    effect::install();
    hook::install();
    expression::install();
    //frame::install();
}