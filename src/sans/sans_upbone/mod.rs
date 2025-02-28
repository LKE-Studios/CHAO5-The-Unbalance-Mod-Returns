mod game;
mod effect;
mod status;

pub fn install() {
    game::install();    
    effect::install();
    status::install();
}