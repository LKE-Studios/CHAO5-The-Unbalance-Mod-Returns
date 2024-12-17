mod maskedman;
mod maskedman_lightning;
mod maskedman_pkstarstorm;
mod maskedman_pkthunder;

pub fn install() {
    maskedman::install(); 
    maskedman_lightning::install(); 
    maskedman_pkstarstorm::install(); 
    maskedman_pkthunder::install(); 
}