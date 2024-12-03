mod Regular;
mod Break;
mod Die;

pub fn install() {
    Regular::install();
    Break::install();
    Die::install();
}