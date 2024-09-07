mod master;
mod master_arrow1;
mod master_arrow2;
mod master_axe;

pub fn install() {
    master::install();  
    master_arrow1::install();      
    master_arrow2::install();    
    master_axe::install();  
}