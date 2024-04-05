mod shizue;
mod shizue_balloon;
mod shizue_bullet;
mod shizue_clayrocket;
mod shizue_office;
mod shizue_pot;
mod shizue_trafficsign;

pub fn install() {
    shizue::install();
    shizue_balloon::install();
    shizue_bullet::install();
    shizue_clayrocket::install();
    shizue_office::install();
    shizue_pot::install();
    shizue_trafficsign::install();
}