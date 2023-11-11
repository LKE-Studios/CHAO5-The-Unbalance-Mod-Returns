mod buddy;
mod buddy_bullet;
mod buddy_pad;

pub fn install() {
    buddy::install(&mut Agent::new("buddy")); 
    buddy_bullet::install(&mut Agent::new("buddy_bullet"));
    buddy_pad::install(&mut Agent::new("buddy_pad"));
    smashline::add_param_object("buddy", "param_glide");
}