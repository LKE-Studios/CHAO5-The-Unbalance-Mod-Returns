mod duckhunt;
mod duckhunt_clay;
mod duckhunt_gunmanbullet;
mod duckhunt_can;

pub fn install() {
    duckhunt::install(); 
    duckhunt_clay::install(); 
    duckhunt_gunmanbullet::install(); 
    duckhunt_can::install();
}