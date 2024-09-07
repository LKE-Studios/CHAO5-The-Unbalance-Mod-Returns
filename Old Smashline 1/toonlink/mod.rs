mod game;
mod sound;
mod status;

pub fn install() {
    game::install();  
    sound::install();  
    status::install();
}