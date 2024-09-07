mod GlideStart;
mod Glide;
mod SpecialN;
mod SpecialNSpin;
mod SpecialNEnd;
mod SpecialS;
mod SpecialSEnd;
mod SpecialHi;
mod SpecialHiLoop;
mod SpecialLw;
mod SpecialLwAttack;
mod SpecialLwEnd;
mod SpecialGuard;

pub fn install() {
    GlideStart::install();
    Glide::install();
    SpecialN::install();
    SpecialNSpin::install();
    SpecialNEnd::install();
    SpecialS::install();
    SpecialSEnd::install();
    SpecialHi::install(); 
    SpecialHiLoop::install(); 
    SpecialLw::install(); 
    SpecialLwAttack::install();
    SpecialLwEnd::install();
    SpecialGuard::install(); 
}