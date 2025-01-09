mod game;
mod frame;
mod effect;
mod sound;
mod status;

pub fn install() {
    game::install();    
    frame::install();
    effect::install();
    sound::install();
    status::install();
}