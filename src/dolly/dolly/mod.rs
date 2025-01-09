mod game;
pub mod frame;
mod sound;

pub fn install() {
    game::install();    
    frame::install();
    sound::install();
}