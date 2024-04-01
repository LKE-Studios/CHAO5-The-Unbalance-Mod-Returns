mod rosetta;
mod rosetta_powerstar;
mod rosetta_starpiece;
mod rosetta_tico;

pub fn install() {
    rosetta::install();
    rosetta_powerstar::install();
    rosetta_starpiece::install();
    rosetta_tico::install();
}