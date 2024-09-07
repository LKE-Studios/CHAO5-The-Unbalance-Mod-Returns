mod game;
mod status;
mod frame;

pub fn install() {
    game::install();
    status::install();
    frame::install();
}