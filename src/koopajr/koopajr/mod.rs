mod game;
mod sound;
mod frame;

pub fn install() {
    game::install();
    sound::install();
    frame::install();   
}