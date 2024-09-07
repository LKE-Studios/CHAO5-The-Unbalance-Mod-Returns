mod toonlink;
mod toonlink_bowarrow;
mod toonlink_boomerang;

pub fn install() {
    toonlink::install();  
    toonlink_bowarrow::install();  
    toonlink_boomerang::install();
}