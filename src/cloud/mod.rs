mod cloud;
mod cloud_wave;

pub fn install() {
    cloud::install(&mut Agent::new("cloud")); 
    claus_wave::install(&mut Agent::new("cloud_wave")); 
}