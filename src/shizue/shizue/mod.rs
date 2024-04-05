mod game;
mod frame;
mod sound;
mod effect;
mod status;

pub fn install() {
    game::install();   
    frame::install(); 
    sound::install();
    effect::install();
    status::install();
}