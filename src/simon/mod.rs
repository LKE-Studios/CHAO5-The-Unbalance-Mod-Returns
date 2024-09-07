mod simon;
mod simon_axe;
mod simon_cross;
mod simon_whip;

pub fn install() {
    simon::install();
    simon_axe::install();
    simon_cross::install();
    simon_whip::install();
}