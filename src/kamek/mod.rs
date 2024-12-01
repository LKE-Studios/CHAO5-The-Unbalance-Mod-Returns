mod kamek;
mod kamek_book;
mod kamek_fire;
mod kamek_fireball;
mod kamek_pinkmagic;
mod kamek_finalmagic;

pub fn install() {
    kamek::install(); 
    kamek_book::install(); 
    kamek_fire::install(); 
    kamek_fireball::install(); 
    kamek_pinkmagic::install(); 
    kamek_finalmagic::install();
    smashline::clone_weapon("mario", *WEAPON_KIND_MARIO_FIREBALL, "ness", "fireball", false);
    smashline::clone_weapon("kirby", *WEAPON_KIND_KIRBY_FINALCUTTERSHOT, "ness", "pinkmagic", false);
    smashline::clone_weapon("rockman", *WEAPON_KIND_ROCKMAN_HARDKNUCKLE, "ness", "book", false);
    smashline::clone_weapon("luigi", *WEAPON_KIND_LUIGI_FIREBALL, "ness", "finalmagic", false);
}