mod game;
mod sound;
mod effect;

pub fn install() {
    game::install(); 
    sound::install();  
    effect::install(); 
}