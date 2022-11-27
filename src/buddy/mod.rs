<<<<<<< HEAD
mod game;
mod effect;
mod frame;
//mod status;
mod sound;

pub fn install() {
    game::install();
    effect::install();    
    frame::install();
    //status::install();
    sound::install();
=======
mod game;
mod effect;
mod frame;
mod status;
mod sound;

pub fn install() {
    game::install();
    effect::install();    
    frame::install();
    status::install();
    sound::install();
>>>>>>> 70e591ed528511fa22d74147c05b77221fd7f3d5
}