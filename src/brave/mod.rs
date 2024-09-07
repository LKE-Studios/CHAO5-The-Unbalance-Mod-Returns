mod brave;
mod brave_crash;
mod brave_deathball;
mod brave_explosion;
mod brave_fireball;
mod brave_flash;
mod brave_sleep;
mod brave_tornado;

pub fn install() {
    brave::install(); 
    brave_crash::install();
    brave_deathball::install(); 
    brave_explosion::install();
    brave_fireball::install();
    brave_flash::install();
    brave_sleep::install();
    brave_tornado::install();
}