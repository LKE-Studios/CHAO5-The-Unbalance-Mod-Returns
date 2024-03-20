mod ness;
mod ness_pkfire;
mod ness_pkstarstorm;
mod ness_pkthunder;
mod ness_yoyohead;

pub fn install() {
    ness::install(); 
    ness_pkfire::install(); 
    ness_pkstarstorm::install(); 
    ness_pkthunder::install(); 
    ness_yoyohead::install();
}