mod ken;
mod ken_hadoken;
mod ken_shinryuken;

pub fn install() {
    ken::install();
    ken_hadoken::install();
    ken_shinryuken::install();
}
