mod game;
mod effect;
mod frame;
mod sound;

pub fn install() {
    game::install();
    effect::install();
    frame::install();
    sound::install();    
}