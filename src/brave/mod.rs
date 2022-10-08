mod game;
mod frame;
mod effect;

pub fn install() {
    game::install();    
    frame::install();
    effect::install();
}