mod elight;
mod elight_spreadbullet;
mod elight_bunshin;
mod elight_exprosiveshot;
mod elight_beam;

pub fn install() {
    elight::install(); 
    elight_spreadbullet::install(); 
    elight_bunshin::install(); 
    elight_exprosiveshot::install(); 
    elight_beam::install(); 
}