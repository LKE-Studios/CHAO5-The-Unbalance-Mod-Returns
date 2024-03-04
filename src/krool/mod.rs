mod krool;
mod krool_ironball;
mod krool_spitball;
mod krool_backpack;
mod krool_crown;

pub fn install() {
    krool::install();
    krool_ironball::install();  
    krool_spitball::install();
    krool_backpack::install();
    krool_crown::install();
}