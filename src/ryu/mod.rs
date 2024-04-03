mod ryu;
mod ryu_hadoken;
mod ryu_shinkuhadoken;

pub fn install() {
    ryu::install();
    ryu_hadoken::install();
    ryu_shinkuhadoken::install();
}
