mod SpecialHi;
pub mod SpecialHiFly;
mod SpecialHiFlyEnd;
mod Glide;

pub fn install() {
    SpecialHi::install();
    SpecialHiFly::install();
    SpecialHiFlyEnd::install();
    Glide::install();
}