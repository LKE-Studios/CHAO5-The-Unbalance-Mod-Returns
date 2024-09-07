mod sheik;
mod sheik_fusin;
mod sheik_needle;

pub fn install() {
    sheik::install();
    sheik_fusin::install();
    sheik_needle::install();
}