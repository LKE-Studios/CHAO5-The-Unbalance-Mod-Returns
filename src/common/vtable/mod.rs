use crate::imports::BuildImports::*; 

mod demon;
mod ganon;

pub fn install() {
    demon::install();
    ganon::install();
}