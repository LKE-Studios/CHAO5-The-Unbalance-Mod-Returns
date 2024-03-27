mod pickel;
mod pickel_fire;
mod pickel_forge;
mod pickel_melt;
mod pickel_meltobject;
mod pickel_trolley;

pub fn install() {
    pickel::install(); 
    pickel_fire::install();
    pickel_forge::install();
    pickel_melt::install();
    pickel_meltobject::install();
    pickel_trolley::install();
}