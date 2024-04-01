mod rockman;
mod rockman_airshooter;
mod rockman_chargeshot;
mod rockman_crashbomb;
mod rockman_hardknuckle;
mod rockman_leafshield;
mod rockman_rockbuster;
mod rockman_rushcoil;

pub fn install() {
    rockman::install(); 
    rockman_airshooter::install(); 
    rockman_chargeshot::install(); 
    rockman_crashbomb::install();
    rockman_hardknuckle::install();
    rockman_leafshield::install();
    rockman_rockbuster::install(); 
    rockman_rushcoil::install(); 
}