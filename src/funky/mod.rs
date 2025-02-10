use crate::imports::BuildImports::*;

mod funky;
mod funky_boot;

pub fn install() {
    funky::install();
    funky_boot::install();
    smashline::clone_weapon("luigi", *WEAPON_KIND_LUIGI_FIREBALL, "donkey", "boot", false);
    smashline::add_param_object("donkey", "param_boot");
}