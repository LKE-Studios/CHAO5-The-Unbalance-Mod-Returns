mod Wait;
mod SquatWait;
mod Walk;
mod Turn;
mod TurnDash;
mod Landing;
mod LandingLight;
mod SpecialS;
mod SpecialSDive;
mod SpecialSLanding;
mod SpecialFAttack;
mod SpecialFEnd;
mod SpecialSCommand;
mod SpecialLw;
mod SpecialLwGuard;
mod SpecialLwAttack;
mod SpecialLwJump;
mod SpecialLwSpecial;

pub fn install() {
    Wait::install();
    SquatWait::install();
    Walk::install();
    Turn::install();
    TurnDash::install();
    Landing::install();
    LandingLight::install();
    SpecialS::install();
    SpecialSDive::install();
    SpecialSLanding::install();
    SpecialFAttack::install();
    SpecialFEnd::install();
    SpecialSCommand::install();
    SpecialLw::install();
    SpecialLwGuard::install();
    SpecialLwAttack::install();
    SpecialLwJump::install();
    SpecialLwSpecial::install();
}

