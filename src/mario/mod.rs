mod mario;
mod mario_fireball;
mod mario_hugeflame;

pub fn install() {
    mario::install(); 
    mario_fireball::install();
    mario_hugeflame::install();
}