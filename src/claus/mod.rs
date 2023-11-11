mod claus;
mod claus_pkfire;
mod claus_pkstarstorm;
mod claus_pkthunder;

pub fn install() {
    claus::install(&mut Agent::new("lucas")); 
    claus_pkfire::install(&mut Agent::new("lucas_pkfire")); 
    claus_pkstarstorm::install(&mut Agent::new("lucas_pkstarstorm")); 
    claus_pkthunder::install(&mut Agent::new("lucas_pkthunder")); 
}