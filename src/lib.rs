#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod captain;
mod donkey;
mod falco;
mod fox;
mod ganon;
mod kirby;
mod koopa;
mod link;
mod luigi;
mod mario;
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
    falco::install();
    // custom::install();
}