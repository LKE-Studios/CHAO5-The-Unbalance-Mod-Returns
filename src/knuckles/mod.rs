use crate::imports::BuildImports::*;

mod knuckles;
mod knuckles_gimmickjump;
mod knuckles_supersonic;

pub static mut KNUCKLES_MERIKOMI_EFFECT_HANDLE : [i32; 8] = [0; 8];

pub fn install() {
    knuckles::install();   
    knuckles_gimmickjump::install();
    knuckles_supersonic::install();
}
