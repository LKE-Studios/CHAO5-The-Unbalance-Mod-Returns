mod SpecialHiStopCeil;
mod SpecialHiStopWall;
mod SpecialHiChargeF;
//mod SpecialHiEnd;

pub fn install() {
    SpecialHiStopCeil::install();
    SpecialHiStopWall::install();
    SpecialHiChargeF::install();
    //SpecialHiEnd::install();
}