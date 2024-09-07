mod szerosuit;
mod szerosuit_paralyzer_bullet;
mod szerosuit_reticle;

pub fn install() {
    szerosuit::install();  
    szerosuit_paralyzer_bullet::install(); 
    szerosuit_reticle::install();  
}