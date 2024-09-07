mod nana;
mod nana_iceberg;
mod nana_iceshot;
mod nana_iceberghit;
mod nana_whitebear;
mod nana_blizzard;
mod nana_condor;

pub fn install() {
    nana::install(); 
    nana_iceberg::install(); 
    nana_iceshot::install(); 
    nana_iceberghit::install(); 
    nana_whitebear::install();
    nana_blizzard::install(); 
    nana_condor::install();
}