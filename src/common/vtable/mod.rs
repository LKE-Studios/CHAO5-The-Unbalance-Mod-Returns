use crate::imports::BuildImports::*; 

mod demon;
mod donkey;
mod edge;
mod ganon;

pub fn install() {
    demon::install();
    donkey::install();
    edge::install();
    ganon::install();
}