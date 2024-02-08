mod JumpAerial;
mod Fly;
pub mod GlideStart;
pub mod Glide;
pub mod GlideAttack;
pub mod GlideEnd;
pub mod GlideLanding;
mod Sub_Glide_checks;
mod Catch;
mod Guard;
mod Dash;
mod AttackDash;
mod Edge_Cancel;
mod SpecialHi;
mod EscapeAir;
mod Rebirth;
//mod Dead;

pub fn install() {
    JumpAerial::install();
    Fly::install();
    GlideStart::install();
    Glide::install();
    GlideAttack::install();
    GlideEnd::install();
    GlideLanding::install();
    Sub_Glide_checks::install();
    Catch::install();
    Guard::install();
    Dash::install();
    AttackDash::install();
    Edge_Cancel::install();
    SpecialHi::install();
    EscapeAir::install();
    Rebirth::install();
    //Dead::install();
}