mod eflame;
mod eflame_esword;
mod eflame_blazepillar;
mod eflame_firepillar;

pub fn install() {
    eflame::install(&mut Agent::new("eflame")); 
    eflame_esword::install(&mut Agent::new("eflame_esword")); 
    eflame_blazepillar::install(&mut Agent::new("eflame_blazepillar")); 
    eflame_firepillar::install(&mut Agent::new("eflame_firepillar")); 
}