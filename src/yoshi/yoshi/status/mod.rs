mod Guard;
mod GuardOn;
mod GuardDamage;

pub fn install() {
    Guard::install();  
    GuardOn::install();  
    GuardDamage::install();  
}