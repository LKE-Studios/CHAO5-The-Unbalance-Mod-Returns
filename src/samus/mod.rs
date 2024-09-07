mod samus;
mod samus_bomb;
mod samus_cshot;
mod samus_laser;
mod samus_laser2;
mod samus_laser3;
mod samus_missile;
mod samus_supermissile;

pub fn install() {
    samus::install(); 
    samus_bomb::install();    
    samus_cshot::install();  
    samus_laser::install();  
    samus_laser2::install();
    samus_laser3::install();    
    samus_missile::install(); 
    samus_supermissile::install();    
}