mod Glide;
//mod SpecialHiRush;
mod SpecialHiRushEnd;
mod JumpAerial;

pub fn install() {
    Glide::install();
    //SpecialHiRush::install();
    SpecialHiRushEnd::install();
    JumpAerial::install();
}