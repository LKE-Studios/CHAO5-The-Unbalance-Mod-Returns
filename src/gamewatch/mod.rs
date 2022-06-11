mod game;
mod effect;
mod sound;

pub fn install() {
    game::install();
    effect::install();
    sound::install();    
}