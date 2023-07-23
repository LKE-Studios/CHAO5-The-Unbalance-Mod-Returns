mod game;
mod effect;
mod frame;
mod sound;
//pub mod status;
pub mod param;

pub fn install() {
    game::install();    
    effect::install();
    frame::install();
    sound::install();
    //status::install();
}