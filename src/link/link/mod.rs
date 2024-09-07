mod game;
mod effect;
mod sound;
mod status;
mod expression;
mod frame;
mod init;

pub fn install() {
    game::install();  
    effect::install();  
    sound::install();
    status::install(); 
    expression::install(); 
    frame::install();
    init::install();
}