mod elight;
mod elight_spreadbullet;
mod elight_bunshin;
mod elight_exprosiveshot;
mod elight_beam;

pub fn install() {
    elight::install(&mut Agent::new("elight")); 
    elight_spreadbullet::install(&mut Agent::new("elight_spreadbullet")); 
    elight_blazepillar::install(&mut Agent::new("elight_bunshin")); 
    elight_exprosiveshot::install(&mut Agent::new("elight_exprosiveshot")); 
    elight_beam::install(&mut Agent::new("elight_beam")); 
}