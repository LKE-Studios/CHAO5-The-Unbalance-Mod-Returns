mod game;
mod effect;
mod frame;
mod sound;
mod init;
pub mod status;

pub fn install() {
    game::install();    
    effect::install();
    frame::install();
    sound::install();
    init::install();
    status::install();
}