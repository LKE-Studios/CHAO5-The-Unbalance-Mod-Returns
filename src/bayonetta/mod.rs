mod bayonetta;
mod bayonetta_specialn_bullet;
mod bayonetta_wickedweavearm;
mod bayonetta_wickedweaveleg;

pub fn install() {
    bayonetta::install(&mut Agent::new("bayonetta")); 
    bayonetta_specialn_bullet::install(&mut Agent::new("bayonetta_specialn_bullet"));  
    bayonetta_wickedweavearm::install(&mut Agent::new("bayonetta_wickedweavearm"));  
    bayonetta_wickedweaveleg::install(&mut Agent::new("bayonetta_wickedweaveleg"));
}