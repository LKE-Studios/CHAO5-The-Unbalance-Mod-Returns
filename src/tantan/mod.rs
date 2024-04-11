mod tantan;
mod tantan_beam;
mod tantan_punch1;
mod tantan_punch2;
mod tantan_punch3;
mod tantan_ring;

pub fn install() {
    tantan::install();   
    tantan_beam::install();
    tantan_punch1::install(); 
    tantan_punch2::install(); 
    tantan_punch3::install(); 
    tantan_ring::install(); 
}