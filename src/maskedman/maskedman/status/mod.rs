pub mod SpecialN;
pub mod SpecialNDash;
mod SpecialNEnd;
pub mod SpecialHi;
pub mod SpecialHiHold;
pub mod SpecialHiEnd;
mod SpecialLw;
mod AirLasso;

pub fn install() {
    SpecialN::install(); 
    SpecialNDash::install(); 
    SpecialNEnd::install();
    SpecialHi::install();
    SpecialHiHold::install(); 
    SpecialHiEnd::install(); 
    SpecialLw::install();
    AirLasso::install();
}