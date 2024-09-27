use crate::imports::BuildImports::*;

mod metaknight;
mod metaknight_galaxiabeam;

pub fn install() {
    metaknight::install(); 
    metaknight_galaxiabeam::install(); 
    smashline::add_param_object("metaknight", "param_glide");
    smashline::add_param_object("metaknight", "param_meta_power");
    smashline::add_param_object("metaknight", "param_galaxiabeam");
    smashline::clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "metaknight", "galaxiabeam", false);
    smashline::update_weapon_count(*WEAPON_KIND_KOOPAJR_CANNONBALL, 5);
}