mod SpecialN;
mod SpecialNOverheat;
mod SpecialS;

pub fn install() {
    SpecialN::install();
    SpecialNOverheat::install();
    SpecialS::install();
}