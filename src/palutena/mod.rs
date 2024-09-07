mod palutena;
mod palutena_autoaimbullet;
mod palutena_beam;
mod palutena_blackhole;
mod palutena_explosiveflame;
mod palutena_reflectionboard;

pub fn install() {
    palutena::install();    
    palutena_autoaimbullet::install(); 
    palutena_beam::install(); 
    palutena_blackhole::install();
    palutena_explosiveflame::install();
    palutena_reflectionboard::install();
    smashline::add_param_object("palutena", "param_glide");
    smashline::add_param_object("palutena", "param_divine_power_up");
}