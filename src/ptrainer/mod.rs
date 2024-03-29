mod ptrainer;
mod ptrainer_pzenigame;
mod ptrainer_pfushigisou;

pub fn install() {
    ptrainer::install();
    ptrainer_pzenigame::install();
    ptrainer_pfushigisou::install();
}