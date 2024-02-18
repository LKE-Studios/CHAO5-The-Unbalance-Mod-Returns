mod gamewatch;
mod gamewatch_bomb;
mod gamewatch_breath;
mod gamewatch_food;
mod gamewatch_octopus;

pub fn install() {
    gamewatch::install(); 
    gamewatch_bomb::install();
    gamewatch_breath::install();
    gamewatch_food::install();
    gamewatch_octopus::install();
}