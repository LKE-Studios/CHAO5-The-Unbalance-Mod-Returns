mod game;
mod effect;
mod status;
mod frame;

pub fn install() {
    game::install();
    effect::install(); 
    status::install(); 
    frame::install();
}