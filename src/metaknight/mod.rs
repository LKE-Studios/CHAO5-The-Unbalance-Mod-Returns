use crate::imports::BuildImports::*;

mod metaknight;
mod metaknight_galaxiabeam;

pub fn install() {
    metaknight::install(); 
    metaknight_galaxiabeam::install(); 
    smashline::add_param_object("metaknight", "param_glide");
    smashline::add_param_object("metaknight", "param_meta_power");
    smashline::add_param_object("metaknight", "param_galaxiabeam");
    smashline::add_param_object("metaknight", "param_critical");
    smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "metaknight", "galaxiabeam", false);
}