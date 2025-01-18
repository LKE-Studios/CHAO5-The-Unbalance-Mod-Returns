use crate::imports::BuildImports::*;

mod ridley;
mod ridley_breath;

pub fn install() {
    ridley::install();
    ridley_breath::install();
    smashline::add_param_object("ridley", "param_glide");
    smashline::update_weapon_count(*WEAPON_KIND_RIDLEY_BREATH, 12);
}