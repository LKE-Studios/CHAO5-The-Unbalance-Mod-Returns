mod SpecialHi;
mod SpecialHi2;
mod SpecialHi2Landing;
mod SpecialGuard;
mod Glide;

pub fn install() {
    SpecialHi::install();
    SpecialHi2::install();
    SpecialHi2Landing::install();
    SpecialGuard::install();
    Glide::install();
}