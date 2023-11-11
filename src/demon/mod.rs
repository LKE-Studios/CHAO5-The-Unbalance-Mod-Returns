mod demon;
mod demon_blaster;
mod demon_blasterchest;

pub fn install() {
    demon::install(&mut Agent::new("demon")); 
    demon_blaster::install(&mut Agent::new("demon_blaster"));
    demon_blasterchest::install(&mut Agent::new("demon_blasterchest"));
}