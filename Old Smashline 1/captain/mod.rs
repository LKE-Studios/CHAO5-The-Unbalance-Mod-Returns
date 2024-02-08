mod game;
mod frame;
mod status;

pub fn install() {
    game::install();   
    frame::install(); 
    status::install();
}