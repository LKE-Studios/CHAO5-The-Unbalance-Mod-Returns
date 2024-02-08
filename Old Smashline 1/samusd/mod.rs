mod game;
mod effect;
mod sound;
mod expression;

pub fn install() {
    game::install();   
    effect::install(); 
    sound::install();
    expression::install();
}