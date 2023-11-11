mod dedede;
mod dedede_gordo;
mod dedede_star;

pub fn install() {
    dedede::install(&mut Agent::new("dedede")); 
    dedede_gordo::install(&mut Agent::new("dedede_gordo"));
    dedede_star::install(&mut Agent::new("dedede_star"));
}