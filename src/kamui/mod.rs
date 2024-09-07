mod kamui;
mod kamui_spearhand;
mod kamui_dragonhand;
mod kamui_ryusensya;
mod kamui_waterdragon;

pub fn install() {
    kamui::install();
    kamui_spearhand::install();    
    kamui_dragonhand::install(); 
    kamui_ryusensya::install();
    kamui_waterdragon::install();
}