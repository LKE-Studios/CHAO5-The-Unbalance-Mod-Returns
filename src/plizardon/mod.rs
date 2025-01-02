use crate::imports::BuildImports::*;

mod plizardon;
mod plizardon_breath;
mod plizardon_daimonji;
mod plizardon_explosion;
mod plizardon_rock;
mod plizardon_rockstone;

pub fn install() {
    plizardon::install(); 
    plizardon_breath::install();
    plizardon_daimonji::install();
    plizardon_explosion::install();
    plizardon_rock::install();
    plizardon_rockstone::install();
    smashline::add_param_object("plizardon", "param_glide");
    smashline::add_param_object("plizardon", "param_special_hi_2");
    smashline::add_param_object("plizardon", "param_special_guard");
    smashline::add_param_object("plizardon", "param_rock");
    smashline::add_param_object("plizardon", "param_rockstone");
    smashline::add_param_object("plizardon", "param_critical");
    smashline::clone_weapon("link", *WEAPON_KIND_LINK_BOOMERANG, "plizardon", "rock", false);
    smashline::clone_weapon("sheik", *WEAPON_KIND_SHEIK_NEEDLE, "plizardon", "rockstone", false);
}