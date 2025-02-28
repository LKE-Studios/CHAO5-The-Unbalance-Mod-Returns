mod Wait;
mod ShoulderStart;
mod SpecialN;
mod SpecialNLoop;
mod SpecialNCancel;
mod SpecialNEnd;
mod SpecialS;
mod SpecialSFly;
mod SpecialSEnd;
mod SpecialSLanding;
mod SpecialHi;
mod SpecialHi_C2;
mod SpecialLw;
mod SpecialLwMusic;
mod SpecialLwFlip;
mod SpecialLwJump;
mod SpecialLwPose;

pub fn install() {
    Wait::install();
    ShoulderStart::install();
    SpecialN::install();
    SpecialNLoop::install();
    SpecialNCancel::install();
    SpecialNEnd::install();
    SpecialS::install();
    SpecialSFly::install();
    SpecialSEnd::install();
    SpecialSLanding::install();
    SpecialHi::install();
    SpecialHi_C2::install();
    SpecialLw::install();
    SpecialLwMusic::install();
    SpecialLwFlip::install();    
    SpecialLwJump::install();
    SpecialLwPose::install();
}