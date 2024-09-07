mod popo;
mod popo_iceberg;
mod popo_iceshot;
mod popo_iceberghit;
mod popo_whitebear;
mod popo_blizzard;
mod popo_condor;

pub fn install() {
    popo::install(); 
    popo_iceberg::install(); 
    popo_iceshot::install(); 
    popo_iceberghit::install(); 
    popo_whitebear::install();
    popo_blizzard::install(); 
    popo_condor::install();
}