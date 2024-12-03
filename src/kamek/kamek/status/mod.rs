mod SpecialNHold;
mod SpecialNFire;
mod SpecialS;
mod SpecialHi;
mod SpecialHiEnd;
mod SpecialHiHold;
mod SpecialLw;

pub fn install() {
    SpecialNHold::install();
    SpecialNFire::install();
    SpecialS::install();
    SpecialHi::install();
    SpecialHiEnd::install();
    SpecialHiHold::install();
    SpecialLw::install();
}