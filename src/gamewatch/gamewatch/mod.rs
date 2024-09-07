mod game;
mod effect;
mod frame;
mod sound;
mod status;

pub fn install() {
    game::install();
    effect::install();
    frame::install();
    sound::install();   
    status::install(); 
}