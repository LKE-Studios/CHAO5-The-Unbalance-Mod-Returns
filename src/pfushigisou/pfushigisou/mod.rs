mod game;
mod effect;
mod sound;
mod expression;
pub mod status;
mod frame;
mod init;

pub fn install() {
    game::install();
    effect::install(); 
    sound::install();
    expression::install();
    status::install();
    frame::install();
    init::install();
}