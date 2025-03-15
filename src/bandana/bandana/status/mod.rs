mod JumpAerial;
mod Attack100;
mod AttackHi4;
mod AttackAir;
mod CatchPull;
mod CatchWait;
mod CatchDashPull;
mod SpecialN;
mod SpecialSCharge;
mod SpecialHi;
mod SpecialLw;
mod Final;
mod FinalEnd;

pub fn install() {
    JumpAerial::install();
    Attack100::install();
    AttackHi4::install();
    AttackAir::install();
    CatchPull::install();
    CatchWait::install();
    CatchDashPull::install();
    SpecialN::install();
    SpecialSCharge::install();
    SpecialHi::install();
    SpecialLw::install();
    Final::install();
    FinalEnd::install();
}