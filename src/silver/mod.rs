use crate::imports::BuildImports::*;

mod silver;
mod silver_beam;
mod silver_mewtwom;
mod silver_psychobreak;
mod silver_search;
mod silver_wave;

pub fn install() {
    silver::install();
    silver_beam::install();
    silver_mewtwom::install();
    silver_psychobreak::install();
    silver_search::install();
    silver_wave::install();
    smashline::clone_weapon("luigi", *WEAPON_KIND_LUIGI_FIREBALL, "mewtwo", "wave", false);
}
