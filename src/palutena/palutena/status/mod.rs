mod GlideStart;
mod Glide;
mod GlideAttack;
mod GlideEnd;
mod GlideLanding;

pub fn install() {
    GlideStart::install();
    Glide::install();
    GlideAttack::install();
    GlideEnd::install();
    GlideLanding::install();
}