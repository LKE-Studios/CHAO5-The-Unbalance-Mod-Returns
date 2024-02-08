mod daisy;
mod daisy_kinopiospore;

pub fn install() {
    daisy::install(&mut Agent::new("daisy")); 
    daisy_kinopiospore::install(&mut Agent::new("daisy_kinopiospore")); 
}