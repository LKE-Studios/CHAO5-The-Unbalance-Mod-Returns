mod trail;
mod trail_fire;
mod trail_ice;
mod trail_thunder;

pub fn install() {
    trail::install();
    trail_fire::install();
    trail_ice::install();
    trail_thunder::install();
    smashline::add_param_object("trail", "param_glide");
}