mod SpecialHiStopCeil;
mod SpecialHiStopWall;
mod SpecialHiChargeF;

pub fn install() {
    SpecialHiStopCeil::install();
    SpecialHiStopWall::install();
    SpecialHiChargeF::install();
}