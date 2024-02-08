mod falco;
mod falco_blaster_bullet;
mod falco_illusion;

pub fn install() {
    falco::install(&mut Agent::new("falco")); 
    falco_blaster_bullet::install(&mut Agent::new("falco_blaster_bullet"));
    falco_illusion::install(&mut Agent::new("falco_illusion"));
}