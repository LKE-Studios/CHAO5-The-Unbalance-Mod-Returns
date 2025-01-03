mod kirby;
mod kirby_finalcuttershot;

pub fn install() {
    kirby::install();
    kirby_finalcuttershot::install();
    smashline::add_param_object("kirby", "param_special_hi_h");
}