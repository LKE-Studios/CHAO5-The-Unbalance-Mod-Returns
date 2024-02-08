mod gamewatch;
mod gamewatch_bomb;
mod gamewatch_breath;
mod gamewatch_food;
mod gamewatch_octopus;

pub fn install() {
    gamewatch::install(&mut Agent::new("gamewatch")); 
    gamewatch_bomb::install(&mut Agent::new("gamewatch_bomb"));
    gamewatch_breath::install(&mut Agent::new("gamewatch_breath"));
    gamewatch_food::install(&mut Agent::new("gamewatch_food"));
    gamewatch_octopus::install(&mut Agent::new("gamewatch_octopus"));
}