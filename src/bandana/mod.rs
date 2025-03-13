mod bandana;
mod bandana_flare1;
mod bandana_flare2;
mod bandana_flash;
mod bandana_fire;

pub fn install() {
    bandana::install(); 
    bandana_flare1::install(); 
    bandana_flare2::install(); 
    bandana_flash::install(); 
    bandana_fire::install();
    smashline::clone_weapon("younglink", *WEAPON_KIND_YOUNGLINK_BOWARROW, "edge", "spear", false);
    smashline::clone_weapon("toonlink", *WEAPON_KIND_TOONLINK_BOWARROW, "edge", "spear2", false);
    smashline::clone_weapon("link", *WEAPON_KIND_LINK_BOWARROW, "edge", "spear3", false);
    smashline::clone_weapon("master", *WEAPON_KIND_MASTER_ARROW1, "edge", "apple", false);
}