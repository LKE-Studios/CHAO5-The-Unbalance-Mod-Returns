mod pitb;
mod pitb_bowarrow;

pub fn install() {
    pitb::install(); 
    pitb_bowarrow::install();
    smashline::add_param_object("pitb", "param_glide");
}