pub mod SpecialNSearch;
mod Dead;

pub fn install() {
    SpecialNSearch::install();
    Dead::install();
}