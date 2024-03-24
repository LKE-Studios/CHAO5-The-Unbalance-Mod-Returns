pub mod SpecialGuard;
mod SpecialGuardShoot;
mod SpecialGuardEnd;

pub fn install() {
    SpecialGuard::install();
    SpecialGuardShoot::install();
    SpecialGuardEnd::install()
}