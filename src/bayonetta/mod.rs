mod bayonetta;
mod bayonetta_specialn_bullet;
mod bayonetta_wickedweavearm;
mod bayonetta_wickedweaveleg;

pub fn install() {
    bayonetta::install(); 
    bayonetta_specialn_bullet::install();  
    bayonetta_wickedweavearm::install();  
    bayonetta_wickedweaveleg::install();
}