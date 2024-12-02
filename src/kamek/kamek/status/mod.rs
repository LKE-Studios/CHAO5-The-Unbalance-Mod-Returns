mod SpecialNHold;
mod SpecialHi;
mod SpecialHiEnd;
mod SpecialHiAttack;
mod SpecialLw;
mod SpecialLwHold;
mod SpecialLwHit;

pub fn install() {
    SpecialNHold::install();
    SpecialHi::install();
    SpecialHiEnd::install();
    SpecialHiAttack::install();
    SpecialLw::install();
    SpecialLwHold::install();
    SpecialLwHit::install();
}