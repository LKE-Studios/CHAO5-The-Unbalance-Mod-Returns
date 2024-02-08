mod diddy;
mod diddy_barreljet;

pub fn install() {
    diddy::install(&mut Agent::new("diddy")); 
    diddy_barreljet::install(&mut Agent::new("diddy_barreljet"));
}