mod game;
pub mod frame;
mod effect;
mod sound;

pub fn install() {
    game::install(); 
    frame::install();
    effect::install(); 
    sound::install();  
}