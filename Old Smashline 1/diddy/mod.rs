mod game;
mod sound;
mod frame;
mod status;

pub fn install() {
    game::install(); 
    sound::install(); 
    frame::install();  
    status::install();
}