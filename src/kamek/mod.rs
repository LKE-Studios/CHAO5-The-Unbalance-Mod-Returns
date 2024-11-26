mod kamek;
mod kamek_pkbeam;
mod kamek_pkhypnosis;
mod kamek_pkstarstorm;

pub fn install() {
    kamek::install(); 
    kamek_pkbeam::install(); 
    kamek_pkstarstorm::install(); 
    kamek_pkhypnosis::install(); 
    smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "ness", "balloffire", false);
    smashline::clone_weapon("kirby", *WEAPON_KIND_KIRBY_FINALCUTTERSHOT, "ness", "pinkmagic", false);
    smashline::clone_weapon("rockman", *WEAPON_KIND_ROCKMAN_HARDKNUCKLE, "ness", "book", false);
    smashline::clone_weapon("luigi", *WEAPON_KIND_LUIGI_FIREBALL, "ness", "finalmagic", false);
}