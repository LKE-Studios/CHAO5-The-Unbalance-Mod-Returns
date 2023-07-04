mod jump_aerial;
mod fly;
pub mod glide;
mod glide_checks;
pub mod glide_param;
mod catch;
mod guard;
mod perfect_pivot;
mod attackdash;
mod edge_cancel;
mod special_hi;
mod escape_air;

pub fn install() {
    jump_aerial::install();
    fly::install();
    glide::install();
    glide_checks::install();
    catch::install();
    guard::install();
    perfect_pivot::install();
    attackdash::install();
    edge_cancel::install();
    special_hi::install();
    escape_air::install();
}