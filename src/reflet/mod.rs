mod reflet;
mod reflet_thunder;
mod reflet_gigafire;
mod reflet_elwind;

pub fn install() {
    reflet::install();
    reflet_thunder::install();
    reflet_gigafire::install();
    reflet_elwind::install();
    smashline::add_param_object("reflet", "param_uniq_float");
}