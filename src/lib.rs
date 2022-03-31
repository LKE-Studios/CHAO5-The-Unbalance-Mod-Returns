#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]
#![allow(non_snake_case)]

pub static mut FIGHTER_CUTIN_MANAGER : *mut smash::app::FighterCutInManager = 0 as _;
use skyline::nn::ro::LookupSymbol;

mod captain;
mod donkey;
mod falco;
mod fox;
mod gamewatch;
mod ganon;
mod kirby;
mod koopa;
mod link;
mod luigi;
mod mario;
mod mariod;
mod marth;
mod ness;
mod pikachu;
mod purin;
mod samus;
mod yoshi;
// mod custom;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    mario::install();
    donkey::install();
    link::install();
    samus::install();
    yoshi::install();
    kirby::install();
    pikachu::install();
    fox::install();
    luigi::install();
    ness::install();
    captain::install();
    purin::install();
    koopa::install();
    marth::install();
    ganon::install();
    gamewatch::install();
    mariod::install();
    falco::install();
    // custom::install();
    unsafe {
        let mut FighterCutinManager = 0usize;
        LookupSymbol(&mut FighterCutinManager as *mut usize, "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E\0".as_ptr() as _);
        FIGHTER_CUTIN_MANAGER = std::mem::transmute(FighterCutinManager);
    }
}