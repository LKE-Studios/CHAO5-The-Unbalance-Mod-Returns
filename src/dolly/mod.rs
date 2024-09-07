mod dolly;
mod dolly_wave;
mod dolly_burst;

pub fn install() {
    dolly::install(); 
    dolly_wave::install();
    dolly_burst::install();
}