mod krystal;
mod krystal_beam;

pub fn install() {
    krystal::install(); 
    krystal_beam::install();
    smashline::add_param_object("pitb", "param_glide");
}