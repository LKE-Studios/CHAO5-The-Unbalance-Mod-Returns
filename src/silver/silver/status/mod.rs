mod SpecialN;
mod SpecialNHold;
mod SpecialNCancel;
mod SpecialNShoot;
mod SpecialNMax;
pub mod SpecialHi;
mod SpecialHi2;
mod SpecialHi3;
mod SpecialLw;
mod Dead;
//mod Final;

pub fn install() {
    SpecialN::install(); 
    SpecialNHold::install();
    SpecialNCancel::install(); 
    SpecialNShoot::install(); 
    SpecialNMax::install(); 
    SpecialHi::install(); 
    SpecialHi2::install();
    SpecialHi3::install();
    SpecialLw::install(); 
    Dead::install();
    //Final::install(); 
}