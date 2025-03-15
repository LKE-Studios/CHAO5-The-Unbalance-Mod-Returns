mod bandana;
mod bandana_apple;
mod bandana_fire;
mod bandana_spear;
mod bandana_spear2;
mod bandana_spear3;

pub fn install() {
    bandana::install(); 
    bandana_apple::install();
    bandana_fire::install(); 
    bandana_spear::install(); 
    bandana_spear2::install(); 
    bandana_spear3::install(); 
    smashline::clone_weapon("master", *WEAPON_KIND_MASTER_ARROW1, "edge", "apple", false);
    smashline::clone_weapon("younglink", *WEAPON_KIND_YOUNGLINK_BOWARROW, "edge", "spear", false);
    smashline::clone_weapon("toonlink", *WEAPON_KIND_TOONLINK_BOWARROW, "edge", "spear2", false);
    smashline::clone_weapon("link", *WEAPON_KIND_LINK_BOWARROW, "edge", "spear3", false);
}