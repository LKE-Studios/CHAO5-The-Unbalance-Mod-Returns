mod game;
mod status;
mod frame;
mod sound;

pub fn install() {
    game::install();    
    frame::install();
    status::install();
    sound::install();
}
