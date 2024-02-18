mod SpecialLw;
mod SpecialLwWaitStart;
mod SpecialLwWait;
mod SpecialLwShoot;

pub fn install() {
    SpecialLw::install(); 
    SpecialLwWaitStart::install();   
    SpecialLwWait::install();
    SpecialLwShoot::install();   
}