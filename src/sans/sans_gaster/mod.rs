mod game;
mod effect;
mod sound;
mod status;

pub fn install() {
    game::install();    
    effect::install();
    sound::install();
    status::install();
}