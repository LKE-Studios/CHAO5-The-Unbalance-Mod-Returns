mod yoshi;
mod yoshi_star;
mod yoshi_tamago;

pub fn install() {
    yoshi::install();  
    yoshi_star::install();  
    yoshi_tamago::install();
}