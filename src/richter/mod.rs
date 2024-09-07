mod richter;
mod richter_axe;
mod richter_cross;
mod richter_whip;

pub fn install() {
    richter::install();
    richter_axe::install();
    richter_cross::install();
    richter_whip::install();
}