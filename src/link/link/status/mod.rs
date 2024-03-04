mod AttachWall;
mod AscendJumpGround;
mod AscendStart;
mod Ascend;
mod AscendEnd;
mod RevaliGlide;
mod RevaliGlideTurn;
mod RevaliGlideDrop;
mod RevaliGlideLanding;
mod SpecialN;
mod SpecialS;

pub fn install() {
    AttachWall::install();  
    AscendJumpGround::install();
    AscendStart::install();
    Ascend::install();
    AscendEnd::install();
    RevaliGlide::install();
    RevaliGlideTurn::install();
    RevaliGlideDrop::install();
    RevaliGlideLanding::install();
    SpecialN::install();
    SpecialS::install();
}
