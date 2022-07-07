mod game;
mod sound;
mod effect;
//mod hook;

pub fn install() {
    game::install(); 
    sound::install();  
    effect::install();
    //hook::install();
}