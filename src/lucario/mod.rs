mod lucario;
mod lucario_qigong;
mod lucario_auraball;
mod lucario_lucariom;

pub fn install() {
    lucario::install(); 
    lucario_qigong::install();   
    lucario_auraball::install();
    lucario_lucariom::install();
}