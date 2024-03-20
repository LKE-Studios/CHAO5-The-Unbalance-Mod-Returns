mod luigi;
mod luigi_fireball;
mod luigi_plunger;
mod luigi_obakyumu;

pub fn install() {
    luigi::install();
    luigi_fireball::install();
    luigi_plunger::install();
    luigi_obakyumu::install();
}