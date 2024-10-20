mod SpecialNShoot;
pub mod SpecialHi;
mod SpecialHi2;
mod SpecialHi3;
mod SpecialLw;
//mod Final;

pub fn install() {
    SpecialNShoot::install(); 
    SpecialHi::install(); 
    SpecialHi2::install();
    SpecialHi3::install();
    SpecialLw::install(); 
    //Final::install(); 
}