mod SpecialHi;
pub mod SpecialHiFly;
mod SpecialHiFlyTurn;
mod SpecialHiFlyEnd;
mod Glide;

pub fn install() {
    SpecialHi::install();
    SpecialHiFly::install();
    SpecialHiFlyTurn::install();
    SpecialHiFlyEnd::install();
    Glide::install();
}