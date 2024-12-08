use crate::imports::BuildImports::*;

mod kamek;
mod kamek_book;
mod kamek_star;
mod kamek_fireball;
mod kamek_finalmagic;
mod kamek_pinkmagic;

pub fn install() {
    smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "ness", "fire", true);
    smashline::clone_weapon("rockman", *WEAPON_KIND_ROCKMAN_HARDKNUCKLE, "ness", "book", false); 
    smashline::clone_weapon("kirby", *WEAPON_KIND_KIRBY_FINALCUTTERSHOT, "ness", "pinkmagic", false);
    smashline::clone_weapon("luigi", *WEAPON_KIND_LUIGI_FIREBALL, "ness", "finalmagic", false);
    kamek::install(); 
    kamek_book::install();
    kamek_star::install(); 
    kamek_fireball::install(); 
    kamek_finalmagic::install();
    kamek_pinkmagic::install(); 
    smashline::add_param_object("ness", "param_fire");
    smashline::add_param_object("ness", "param_pinkmagic");
    smashline::update_weapon_count(*WEAPON_KIND_NESS_PK_FIRE, 4);
}