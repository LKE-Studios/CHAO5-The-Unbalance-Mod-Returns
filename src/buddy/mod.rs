mod buddy;
mod buddy_bullet;
mod buddy_pad;

pub fn install() {
    buddy::install(); 
    buddy_bullet::install();
    buddy_pad::install();
    smashline::add_param_object("buddy", "param_glide");
}