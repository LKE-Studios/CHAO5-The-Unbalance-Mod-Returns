mod miiswordsman;
mod miiswordsman_tornadoshot;
mod miiswordsman_chakram;
mod miiswordsman_wave;
mod miiswordsman_lightshuriken;

pub fn install() {
    miiswordsman::install();
    miiswordsman_tornadoshot::install(); 
    miiswordsman_chakram::install(); 
    miiswordsman_wave::install();
    miiswordsman_lightshuriken::install();
}