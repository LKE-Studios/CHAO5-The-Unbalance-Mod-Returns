mod game;
mod sound;
mod status;
mod frame;

pub fn install() {
    game::install();
    sound::install();
    status::install();
    frame::install();
}