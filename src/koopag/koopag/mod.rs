mod game;
mod frame;
mod status;
mod effect;
mod expression;
mod sound;
mod init;

pub fn install() {
    game::install();  
    frame::install();  
    status::install();
    effect::install();
    expression::install();
    sound::install();
    init::install();
}