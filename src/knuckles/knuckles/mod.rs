mod game;
mod status;
mod frame;
mod sound;
mod effect;

pub fn install() {
    game::install();    
    frame::install();
    status::install();
    sound::install();
    effect::install();
}
