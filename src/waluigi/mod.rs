mod waluigi;
mod waluigi_diceblock;

pub fn install() {
    waluigi::install();
    waluigi_diceblock::install();
    smashline::clone_weapon("mario", "fireball", "dolly", "diceblock", false);
}
