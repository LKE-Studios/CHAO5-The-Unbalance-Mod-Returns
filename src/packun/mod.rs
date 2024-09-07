mod packun;
mod packun_spikeball;
mod packun_poisonbreath;
mod packun_bosspackun;

pub fn install() {
    packun::install();   
    packun_spikeball::install(); 
    packun_poisonbreath::install();
    packun_bosspackun::install();
}