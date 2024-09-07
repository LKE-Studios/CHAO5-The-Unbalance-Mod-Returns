mod demon;
mod demon_blaster;
mod demon_blasterchest;

pub fn install() {
    demon::install(); 
    demon_blaster::install();
    demon_blasterchest::install();
}