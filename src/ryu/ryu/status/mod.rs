pub mod SpecialN;
mod KamehamehaStart;
mod KamehamehaCharge;
mod KamehamehaFire;

pub fn install() {
    SpecialN::install();
    KamehamehaStart::install();
    KamehamehaCharge::install();
    KamehamehaFire::install();
}