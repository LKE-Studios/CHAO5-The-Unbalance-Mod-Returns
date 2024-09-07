mod koopajr;
mod koopajr_cannonball;
mod koopajr_remainclown;
mod koopajr_batten;

pub fn install() {
    koopajr::install();  
    koopajr_cannonball::install(); 
    koopajr_remainclown::install(); 
    koopajr_batten::install(); 
}