mod edge;
mod edge_flare1;
mod edge_flare2;
mod edge_flash;
mod edge_fire;

pub fn install() {
    edge::install(); 
    edge_flare1::install(); 
    edge_flare2::install(); 
    edge_flash::install(); 
    edge_fire::install();
}