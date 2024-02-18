mod SpecialHi;
pub mod SpecialHiFly;
mod SpecialHiFlyTurn;
mod SpecialHiFlyEnd;

pub fn install() {
    SpecialHi::install();
    SpecialHiFly::install();
    SpecialHiFlyTurn::install();
    SpecialHiFlyEnd::install();
}