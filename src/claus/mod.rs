mod claus;
mod claus_pkfire;
mod claus_pkstarstorm;
mod claus_pkthunder;

pub fn install() {
    claus::install(); 
    claus_pkfire::install(); 
    claus_pkstarstorm::install(); 
    claus_pkthunder::install(); 
}