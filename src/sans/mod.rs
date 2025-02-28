use crate::imports::BuildImports::*; 

mod sans;
mod sans_upbone;
mod sans_downbone;
mod sans_sidebone;
mod sans_gaster;

pub fn install() {
    sans::install();    
    sans_upbone::install(); 
    sans_downbone::install(); 
    sans_sidebone::install();
    sans_gaster::install();
    smashline::clone_weapon("rockman", *WEAPON_KIND_ROCKMAN_ROCKBUSTER, "palutena", "upbone", false);
    smashline::clone_weapon("rockman", *WEAPON_KIND_ROCKMAN_HARDKNUCKLE, "palutena", "downbone", false);
    smashline::clone_weapon("mariod", *WEAPON_KIND_MARIOD_DRCAPSULE, "palutena", "sidebone", false);
    smashline::clone_weapon("koopajr", *WEAPON_KIND_KOOPAJR_CANNONBALL, "palutena", "gaster", false);
    smashline::add_param_object("palutena", "param_upbone");
    smashline::add_param_object("palutena", "param_downbone");
    smashline::add_param_object("palutena", "param_sidebone");
    smashline::add_param_object("palutena", "param_gaster");
    smashline::update_weapon_count(*WEAPON_KIND_KOOPAJR_CANNONBALL, 3);
}