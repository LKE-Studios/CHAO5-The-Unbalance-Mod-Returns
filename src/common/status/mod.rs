mod jump_aerial;
mod fly;
pub mod glide;
mod glide_checks;
mod catch;
mod shield_drop;

pub fn install() {
    jump_aerial::install();
    fly::install();
    glide::install();
    glide_checks::install();
    catch::install();
    shield_drop::install();
}