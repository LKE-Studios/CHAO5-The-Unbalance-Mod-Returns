mod game;
mod effect;
mod frame;
mod sound;
pub mod status;
mod init;

pub fn install() {
    game::install();    
    effect::install();
    frame::install();
    sound::install();
    status::install();
    init::install();
}