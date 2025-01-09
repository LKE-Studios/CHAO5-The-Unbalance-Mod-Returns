mod game;
mod frame;
mod sound;
//mod status;

pub fn install() {
    game::install(); 
    frame::install();
    sound::install();
    //status::install();
}