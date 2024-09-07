mod mariod;
mod mariod_drcapsule;
mod mariod_hugecapsule;

pub fn install() {
    mariod::install();
    mariod_drcapsule::install();
    mariod_hugecapsule::install();
}