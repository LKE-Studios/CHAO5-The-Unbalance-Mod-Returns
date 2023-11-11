mod captain;

pub fn install() {
    captain::install(&mut Agent::new("captain")); 
}