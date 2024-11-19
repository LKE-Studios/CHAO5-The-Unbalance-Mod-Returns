mod kamek;
mod kamek_pkbeam;
mod kamek_pkhypnosis;
mod kamek_pkstarstorm;

pub fn install() {
    kamek::install(); 
    kamek_pkbeam::install(); 
    kamek_pkstarstorm::install(); 
    kamek_pkhypnosis::install(); 
}