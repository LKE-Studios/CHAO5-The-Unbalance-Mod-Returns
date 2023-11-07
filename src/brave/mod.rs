mod brave;
mod brave_crash;
mod brave_deathball;
mod brave_explosion;
mod brave_fireball;
mod brave_flash;
mod brave_sleep;
mod brave_tornado;

pub fn install() {
    brave::install(&mut Agent::new("brave")); 
    brave_crash::install(&mut Agent::new("brave_crash"));
    brave_deathball::install(&mut Agent::new("brave_deathball")); 
    brave_explosion::install(&mut Agent::new("brave_explosion"));
    brave_fireball::install(&mut Agent::new("brave_fireball"));
    brave_flash::install(&mut Agent::new("brave_flash"));
    brave_sleep::install(&mut Agent::new("brave_sleep"));
    brave_tornado::install(&mut Agent::new("brave_tornado"));
}