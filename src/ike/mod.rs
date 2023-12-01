mod game;
mod frame;
mod effect;
mod status;

pub fn install() {
    game::install();    
    frame::install();
    effect::install();
    status::install();
}