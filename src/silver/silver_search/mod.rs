mod effect;
mod game;
mod sound;

pub fn install() { 
    effect::install(); 
    game::install(); 
    sound::install(); 
}