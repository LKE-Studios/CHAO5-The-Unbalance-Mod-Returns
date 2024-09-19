use crate::imports::BuildImports::*;

mod waluigi;
mod waluigi_diceblock;

pub fn install() {
    waluigi::install();
    waluigi_diceblock::install();
    smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "dolly", "diceblock", false);
}
