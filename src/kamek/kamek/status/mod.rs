pub mod SpecialN;
pub mod SpecialNHold;
pub mod SpecialNFire;
mod SpecialS;
mod SpecialHi;
mod SpecialHiEnd;
mod SpecialHiWarp;
mod SpecialLw;

pub fn install() {
    SpecialN::install();
    SpecialNHold::install();
    SpecialNFire::install();
    SpecialS::install();
    SpecialHi::install();
    SpecialHiEnd::install();
    SpecialHiWarp::install();
    SpecialLw::install();
}