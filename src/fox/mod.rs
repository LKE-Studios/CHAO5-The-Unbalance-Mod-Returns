mod fox;
mod fox_blaster_bullet;
mod fox_illusion;
mod fox_arwingshot;

pub fn install() {
    fox::install(&mut Agent::new("fox")); 
    fox_blaster_bullet::install(&mut Agent::new("fox_blaster_bullet"));
    fox_illusion::install(&mut Agent::new("fox_illusion"));
    fox_arwingshot::install(&mut Agent::new("fox_arwingshot"));
}