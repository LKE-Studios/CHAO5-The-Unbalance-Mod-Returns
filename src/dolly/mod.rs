mod dolly;
mod dolly_wave;
mod dolly_burst;

pub fn install() {
    dolly::install(&mut Agent::new("dolly")); 
    dolly_wave::install(&mut Agent::new("dolly_wave"));
    dolly_burst::install(&mut Agent::new("dolly_burst"));
}