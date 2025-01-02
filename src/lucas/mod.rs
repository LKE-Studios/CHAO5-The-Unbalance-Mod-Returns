mod lucas;
mod lucas_pkfire;
mod lucas_pkstarstorm;
mod lucas_pkthunder;

pub fn install() {
    lucas::install(); 
    lucas_pkfire::install(); 
    lucas_pkstarstorm::install(); 
    lucas_pkthunder::install(); 
    smashline::add_param_object("lucas", "param_critical");
}