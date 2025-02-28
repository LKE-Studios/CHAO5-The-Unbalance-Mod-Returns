mod Attack;
mod AttackAir;
mod AttackDash;
mod AttackHi3;
mod AttackLw3;
mod DownStandAttack;
mod SpecialHi;
mod SpecialHi3;
mod SpecialLw;
mod Final;

pub fn install() {
    Attack::install();
    AttackAir::install();
    AttackDash::install();
    AttackHi3::install();
    AttackLw3::install();
    DownStandAttack::install();
    SpecialHi::install();
    SpecialHi3::install();
    SpecialLw::install();
    Final::install();
}