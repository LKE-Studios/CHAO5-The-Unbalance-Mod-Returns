mod SpecialHiStopCeil;
mod SpecialHiStopWall;
mod SpecialHiChargeF;
//mod SpecialHiEnd;
mod Glide;

pub fn install() {
    SpecialHiStopCeil::install();
    SpecialHiStopWall::install();
    SpecialHiChargeF::install();
    //SpecialHiEnd::install();
    Glide::install();
}