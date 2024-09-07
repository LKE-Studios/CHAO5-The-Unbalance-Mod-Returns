mod game;
mod sound;
mod status;
mod expression;

pub fn install() {
    game::install();  
    sound::install();
    status::install(); 
    expression::install(); 
}