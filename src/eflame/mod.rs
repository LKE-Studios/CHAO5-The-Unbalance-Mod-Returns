mod eflame;
mod eflame_esword;
mod eflame_blazepillar;
mod eflame_firepillar;

pub fn install() {
    eflame::install(); 
    eflame_esword::install(); 
    eflame_blazepillar::install(); 
    eflame_firepillar::install(); 
}