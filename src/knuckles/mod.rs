mod knuckles;
mod knuckles_gimmickjump;
mod knuckles_superknuckles;

pub fn install() {
    knuckles::install();   
    knuckles_gimmickjump::install();
    knuckles_supersonic::install();
}
