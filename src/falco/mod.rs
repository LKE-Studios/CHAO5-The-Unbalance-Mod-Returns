mod falco;
mod falco_blaster_bullet;
mod falco_illusion;

pub fn install() {
    falco::install(); 
    falco_blaster_bullet::install();
    falco_illusion::install();
}