mod ganon;
mod ganon_ganond;

pub fn install() {
    ganon::install(&mut Agent::new("ganon")); 
    ganon_ganond::install(&mut Agent::new("ganon_ganond"));
}