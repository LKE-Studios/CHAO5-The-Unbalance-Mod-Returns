mod snake;
mod snake_c4;
mod snake_trenchmortarbullet;
mod snake_nikitamissile;
mod snake_cypher;
mod snake_missile;

pub fn install() {
    snake::install();
    snake_c4::install();
    snake_trenchmortarbullet::install();    
    snake_nikitamissile::install();
    snake_cypher::install();
    snake_missile::install();
}