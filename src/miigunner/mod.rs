mod miigunner;
mod miigunner_attackairf_bullet;
mod miigunner_rapidshot_bullet;
mod miigunner_gunnercharge;
mod miigunner_grenadelauncher;
mod miigunner_flamepillar;
mod miigunner_stealthbomb_s;
mod miigunner_miimissile;
mod miigunner_supermissile;
mod miigunner_bottomshoot;
mod miigunner_groundbomb;
mod miigunner_laser;
mod miigunner_fullthrottle;

pub fn install() {
    miigunner::install();  
    miigunner_attackairf_bullet::install(); 
    miigunner_rapidshot_bullet::install(); 
    miigunner_gunnercharge::install(); 
    miigunner_grenadelauncher::install();
    miigunner_flamepillar::install();
    miigunner_stealthbomb_s::install();
    miigunner_miimissile::install();
    miigunner_supermissile::install();
    miigunner_bottomshoot::install();
    miigunner_groundbomb::install();
    miigunner_laser::install();
    miigunner_fullthrottle::install();
}