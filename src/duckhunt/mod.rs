mod duckhunt;
mod duckhunt_clay;
mod duckhunt_gunmanbullet;
mod duckhunt_can;

pub fn install() {
    duckhunt::install(&mut Agent::new("duckhunt")); 
    duckhunt_clay::install(&mut Agent::new("duckhunt_clay")); 
    duckhunt_gunmanbullet::install(&mut Agent::new("duckhunt_gunmanbullet")); 
    duckhunt_can::install(&mut Agent::new("duckhunt_can")); 
}