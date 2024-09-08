mod SpecialN;
mod SpecialNOverheat;
mod SpecialS;
mod SpecialHi;
mod SpecialHiOverheat;
mod SpecialLw;
mod SpecialLw2;
mod SpecialLwLoop;
mod SpecialLwLanding;
mod SpecialLwAirEnd;

pub fn install() {
    SpecialN::install();
    SpecialNOverheat::install();
    SpecialS::install();
    SpecialHi::install();
    SpecialHiOverheat::install();
    SpecialLw::install();
    SpecialLw2::install();
    SpecialLwLoop::install();
    SpecialLwLanding::install();
    SpecialLwAirEnd::install();
}