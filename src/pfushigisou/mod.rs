use crate::imports::BuildImports::*;

mod pfushigisou;
mod pfushigisou_seed;
mod pfushigisou_leafcutter;

pub fn install() {
    pfushigisou::install(); 
    pfushigisou_seed::install(); 
    pfushigisou_leafcutter::install(); 
    smashline::add_param_object("pfushigisou", "param_special_guard");
    smashline::update_weapon_count(*WEAPON_KIND_PFUSHIGISOU_LEAFCUTTER, 5);
}