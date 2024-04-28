mod younglink;
mod younglink_bowarrow;
mod younglink_boomerang;

pub fn install() {
    younglink::install();    
    younglink_bowarrow::install();
    younglink_boomerang::install();
}