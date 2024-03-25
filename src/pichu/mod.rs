mod pichu;
mod pichu_dengekidama;
mod pichu_kaminari;

pub fn install() {
    pichu::install();    
    pichu_dengekidama::install();
    pichu_kaminari::install();
}