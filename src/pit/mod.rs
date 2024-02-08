mod pit;
mod pit_bowarrow;

pub fn install() {
    pit::install(); 
    pit_bowarrow::install();
    smashline::add_param_object("pit", "param_glide");
    smashline::add_param_object("pit", "param_special_hi_fly");
}