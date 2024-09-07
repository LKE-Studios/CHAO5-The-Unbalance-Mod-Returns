mod link;
mod link_bowarrow;
mod link_boomerang;
mod link_ancientbowarrow;
mod link_parasail;

pub fn install() {
    link::install();  
    link_bowarrow::install(); 
    link_boomerang::install(); 
    link_ancientbowarrow::install(); 
    link_parasail::install(); 
    smashline::add_param_object("link", "param_ascend_revali");
}