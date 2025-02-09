mod funky;

pub fn install() {
    funky::install(); 
    smashline::clone_weapon("luigi", *WEAPON_KIND_LUIGI_FIREBALL, "donkey", "boot", false);
}