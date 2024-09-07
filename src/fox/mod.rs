mod fox;
mod fox_blaster_bullet;
mod fox_illusion;
mod fox_arwingshot;

pub fn install() {
    fox::install(); 
    fox_blaster_bullet::install();
    fox_illusion::install();
    fox_arwingshot::install();
}