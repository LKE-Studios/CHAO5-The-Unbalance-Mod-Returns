mod metaknight;

pub fn install() {
    metaknight::install(); 
    smashline::add_param_object("metaknight", "param_glide");
    smashline::add_param_object("metaknight", "param_meta_power");
}