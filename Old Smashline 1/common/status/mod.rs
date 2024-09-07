mod JumpAerial;
mod Fly;
pub mod Glide;
mod GlideStart;
mod GlideAttack;
mod GlideEnd;
mod GlideLanding;
mod glide_checks;
pub mod glide_param;
mod Catch;
mod Guard;
mod PerfectPivot;
mod AttackDash;
mod edge_cancel;
mod SpecialHi;
mod EscapeAir;
mod Damage;
//mod dead;

pub fn install() {
    JumpAerial::install();
    Fly::install();
    GlideStart::install();
    Glide::install();
    GlideAttack::install();
    GlideEnd::install();
    GlideLanding::install();
    glide_checks::install();
    Catch::install();
    Guard::install();
    PerfectPivot::install();
    AttackDash::install();
    edge_cancel::install();
    SpecialHi::install();
    EscapeAir::install();
    Damage::install();
    //dead::install();
}