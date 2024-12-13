pub mod SpecialN;
pub mod SpecialNHold;
pub mod SpecialNFire;
pub mod SpecialS;
pub mod SpecialHi;
pub mod SpecialHiEnd;
pub mod SpecialHiWarp;
pub mod SpecialLw;

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