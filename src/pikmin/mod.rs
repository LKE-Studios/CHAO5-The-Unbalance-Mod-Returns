mod pikmin;
mod pikmin_dolfin;
mod pikmin_pikmin; 

pub fn install() {
    pikmin::install();  
    pikmin_dolfin::install();  
    pikmin_pikmin::install(); 
}