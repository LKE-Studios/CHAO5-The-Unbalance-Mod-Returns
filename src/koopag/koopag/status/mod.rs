mod AttackAir;
mod GuardOn;
mod Guard;
mod Dead;
mod ShieldBreakFly;

pub fn install() {
    AttackAir::install();  
    GuardOn::install();
    Guard::install();
    Dead::install();
    ShieldBreakFly::install();
}