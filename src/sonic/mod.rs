mod sonic;
mod sonic_gimmickjump;
mod sonic_supersonic;

pub fn install() {
    sonic::install();   
    sonic_gimmickjump::install();
    sonic_supersonic::install();
}
