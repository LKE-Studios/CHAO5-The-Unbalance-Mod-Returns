use crate::imports::BuildImports::*;

mod maskedman;
mod maskedman_lightning;
mod maskedman_pkstarstorm;
mod maskedman_pkthunder;

pub fn install() {
    maskedman::install(); 
    maskedman_lightning::install(); 
    maskedman_pkstarstorm::install(); 
    maskedman_pkthunder::install(); 
    smashline::update_weapon_count(*WEAPON_KIND_LUCAS_PK_FIRE, 4);
}