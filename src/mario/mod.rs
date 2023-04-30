mod game;
mod frame;
mod sound;
mod effect;

pub fn install() {
    game::install(); 
    frame::install();   
    sound::install();
    effect::install();
}
