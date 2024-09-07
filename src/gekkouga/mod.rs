mod gekkouga;
mod gekkouga_shuriken;
mod gekkouga_water;

pub fn install() {
    gekkouga::install();
    gekkouga_shuriken::install();
    gekkouga_water::install();
}