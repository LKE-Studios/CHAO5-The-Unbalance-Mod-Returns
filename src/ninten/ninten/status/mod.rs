mod AttackHi4;
mod SpecialHi;
mod SpecialHiHold;
mod SpecialHiEnd;
mod SpecialHiAttack;
mod SpecialLw;
mod SpecialLwHold;
mod SpecialLwHit;

pub fn install() {
    AttackHi4::install();
    SpecialHi::install();
    SpecialHiHold::install();
    SpecialHiEnd::install();
    SpecialHiAttack::install();
    SpecialLw::install();
    SpecialLwHold::install();
    SpecialLwHit::install();
}