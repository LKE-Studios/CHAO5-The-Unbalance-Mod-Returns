mod ninten;
mod ninten_pkbeam;
mod ninten_pkhypnosis;
mod ninten_pkstarstorm;

pub fn install() {
    ninten::install(); 
    ninten_pkbeam::install(); 
    ninten_pkstarstorm::install(); 
    ninten_pkhypnosis::install(); 
}