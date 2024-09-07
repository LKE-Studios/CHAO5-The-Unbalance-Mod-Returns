mod SpecialSFall;
mod SpecialSJump;
mod SpecialLw;
mod SpecialLwPlant;

pub fn install() {
    SpecialSFall::install();   
    SpecialSJump::install();   
    SpecialLw::install();  
    SpecialLwPlant::install();  
}