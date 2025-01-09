mod game;
mod effect;
mod sound;
mod status;
mod frame;

pub fn install() {
    game::install();
    effect::install(); 
    sound::install();
    status::install(); 
    frame::install();
}