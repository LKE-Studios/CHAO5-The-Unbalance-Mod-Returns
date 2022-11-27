mod game;
mod effect;
mod frame;
mod status;
mod sound;

pub fn install() {
    game::install();
    effect::install();    
    frame::install();
    status::install();
    sound::install();
}