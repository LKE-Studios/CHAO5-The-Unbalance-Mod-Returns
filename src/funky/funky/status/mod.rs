mod Wait;
mod ShoulderStart;
mod SpecialN;
mod SpecialNLoop;
mod SpecialS;
mod SpecialSFly;
mod SpecialSEnd;
mod SpecialSLanding;
mod SpecialHi;
mod SpecialHi_C2;
mod SpecialLw;

pub fn install() {
    Wait::install();
    ShoulderStart::install();
    SpecialN::install();
    SpecialNLoop::install();
    SpecialS::install();
    SpecialSFly::install();
    SpecialSEnd::install();
    SpecialSLanding::install();
    SpecialHi::install();
    SpecialHi_C2::install();
    SpecialLw::install();
}