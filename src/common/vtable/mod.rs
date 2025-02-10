use crate::imports::BuildImports::*; 

mod demon;
mod donkey;
mod ganon;

pub fn install() {
    demon::install();
    donkey::install();
    ganon::install();
}