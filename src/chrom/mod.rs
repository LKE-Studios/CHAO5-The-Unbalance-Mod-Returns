mod chrom;

pub fn install() {
    chrom::install(&mut Agent::new("chrom")); 
}