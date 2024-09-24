use crate::imports::BuildImports::*;

static mut FLOAT_OFFSET : usize = 0x4e53e0; //13.0.2
static mut INT_OFFSET : usize = 0x4e53a0; //13.0.2

pub extern "C" fn new_fighter_params(_ev: Event) {
    //CLAUS
    param_config::update_float_2(*FIGHTER_KIND_LUCAS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("walk_speed_max"), 0, 1.47));
    param_config::update_float_2(*FIGHTER_KIND_LUCAS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("ground_brake"), 0, 0.045));
    param_config::update_float_2(*FIGHTER_KIND_LUCAS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("dash_speed"), 0, 2.24));
    param_config::update_float_2(*FIGHTER_KIND_LUCAS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("run_speed_max"), 0, 1.9));
    param_config::update_float_2(*FIGHTER_KIND_LUCAS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("air_speed_x_stable"), 0, 1.4883));
    param_config::update_float_2(*FIGHTER_KIND_LUCAS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("dive_speed_y"), 0, 2.35));
    param_config::update_float_2(*WEAPON_KIND_LUCAS_PK_FIRE, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_pkfire"), hash40("speed_ground"), 7.5));

    //KRYSTAL
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("jump_count_max"), 0, 4));
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("wall_jump_type"), 0, 1));
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_n"), hash40("charge_frame"), 0));
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_glide"), hash40("jump_button_hold_glide_frame"), -1));
    return;
}