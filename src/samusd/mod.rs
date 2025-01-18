mod samusd;
mod samusd_bomb;
mod samusd_cshot;
mod samusd_laser;
mod samusd_laser2;
mod samusd_laser3;
mod samusd_missile;
mod samusd_supermissile;

pub fn install() {
    samusd::install(); 
    samusd_bomb::install();    
    samusd_cshot::install();  
    samusd_laser::install();  
    samusd_laser2::install();
    samusd_laser3::install();    
    samusd_missile::install(); 
    samusd_supermissile::install();    
    smashline::add_param_object("samusd", "param_uniq_float");
}