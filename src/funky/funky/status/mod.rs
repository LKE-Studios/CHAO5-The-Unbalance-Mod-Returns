mod Wait;
mod ShoulderStart;
mod SpecialN;
mod SpecialNLoop;
mod SpecialHi;
mod SpecialHi_C2;
mod SpecialLw;

pub fn install() {
    Wait::install();
    ShoulderStart::install();
    SpecialN::install();
    SpecialNLoop::install();
    SpecialHi::install();
    SpecialHi_C2::install();
    SpecialLw::install();
}