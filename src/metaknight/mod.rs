use crate::imports::BuildImports::*;

mod metaknight;
mod metaknight_galaxiabeam;

pub fn install() {
    metaknight::install(); 
    metaknight_galaxiabeam::install(); 
    smashline::add_param_object("metaknight", "param_glide");
    smashline::add_param_object("metaknight", "param_meta_power");
    smashline::add_param_object("metaknight", "param_galaxiabeam");
    smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "metaknight", "galaxiabeam", false);
    smashline::update_weapon_count(*WEAPON_KIND_MARIO_FIREBALL, 6);
}