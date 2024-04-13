mod murabito;
mod murabito_flowerpot;
mod murabito_bowlingball;
mod murabito_firework;
mod murabito_bullet;
mod murabito_balloon;
mod murabito_sprinkling_water;
mod murabito_sprout;
mod murabito_tree;
mod murabito_house;
mod murabito_clayrocket;

pub fn install() {
    murabito::install();
    murabito_bowlingball::install();
    murabito_flowerpot::install();
    murabito_firework::install();
    murabito_bullet::install();
    murabito_balloon::install();
    murabito_sprinkling_water::install();
    murabito_sprout::install();
    murabito_tree::install();
    murabito_house::install();
    murabito_clayrocket::install();
}