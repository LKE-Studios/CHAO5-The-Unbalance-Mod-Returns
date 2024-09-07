pub mod mewtwo;
mod mewtwo_shadowball;
mod mewtwo_bindball;
mod mewtwo_psychobreak;

pub fn install() {
    mewtwo::install(); 
    mewtwo_shadowball::install();
    mewtwo_bindball::install();
    mewtwo_psychobreak::install();
}