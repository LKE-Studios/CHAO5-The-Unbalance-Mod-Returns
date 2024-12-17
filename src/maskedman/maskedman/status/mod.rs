pub mod SpecialN;
pub mod SpecialNDash;
mod SpecialNEnd;
pub mod SpecialHi;
pub mod SpecialHiHold;
mod AirLasso;

pub fn install() {
    SpecialN::install(); 
    SpecialNDash::install(); 
    SpecialNEnd::install();
    SpecialHi::install();
    SpecialHiHold::install(); 
    AirLasso::install();
}