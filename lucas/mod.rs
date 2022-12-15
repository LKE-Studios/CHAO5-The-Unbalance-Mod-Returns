mod game;
mod sound;
mod effect;
mod hook;
//mod frame;

pub fn install() {
    game::install(); 
    sound::install();  
    effect::install();
    hook::install();
    //frame::install();
}