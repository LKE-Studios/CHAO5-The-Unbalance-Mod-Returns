mod jump_aerial;
mod fly;
pub mod glide;
mod glide_checks;

pub fn install() {
    jump_aerial::install();
    fly::install();
    glide::install();
    glide_checks::install();
}