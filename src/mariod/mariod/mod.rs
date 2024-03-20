mod game;
mod sound;
mod effect;
mod frame;

pub fn install() {
    game::install();
    sound::install();
    effect::install();   
    frame::install(); 
}