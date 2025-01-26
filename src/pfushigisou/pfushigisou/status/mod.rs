pub mod SpecialGuard;
mod SpecialGuardShoot;
mod SpecialGuardEnd;
mod SpecialLw;
mod SpecialLwOut;
mod SpecialLwStandby;

pub fn install() {
    SpecialGuard::install();
    SpecialGuardShoot::install();
    SpecialGuardEnd::install();
    SpecialLw::install();
    SpecialLwOut::install();
    SpecialLwStandby::install();
}