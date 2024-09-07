mod pacman;
mod pacman_trampoline;
mod pacman_firehydrant;
mod pacman_firehydrantwater;
mod pacman_bigpacman;

pub fn install() {
    pacman::install();
    pacman_trampoline::install();
    pacman_firehydrant::install();
    pacman_firehydrantwater::install();
    pacman_bigpacman::install();
}