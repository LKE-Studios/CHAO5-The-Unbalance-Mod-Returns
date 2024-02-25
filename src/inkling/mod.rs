mod inkling;
mod inkling_roller;
mod inkling_splash;
mod inkling_splashbomb;
mod inkling_megaphonelaser;

pub fn install() {
    inkling::install();  
    inkling_roller::install();   
    inkling_splash::install(); 
    inkling_splashbomb::install(); 
    inkling_megaphonelaser::install(); 
}