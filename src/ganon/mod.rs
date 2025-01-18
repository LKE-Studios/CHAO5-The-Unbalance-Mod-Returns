mod ganon;
mod ganon_ganond;

pub fn install() {
    ganon::install(); 
    ganon_ganond::install();
    smashline::add_param_object("ganon", "param_attack_air_lw");
    smashline::add_param_object("ganon", "param_uniq_float");
}