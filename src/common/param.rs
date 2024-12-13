use crate::imports::BuildImports::*;

static mut FLOAT_OFFSET : usize = 0x4e53e0; //13.0.2
static mut INT_OFFSET : usize = 0x4e53a0; //13.0.2

//New Fighter Parameters. Sadly config_param.toml has its limitations
extern "C" fn new_fighter_params(_ev: Event) {
    //LINK
    param_config::update_int_2(-*WEAPON_KIND_LINK_PARASAIL, vec![-1].clone(),(hash40("article_use_type"), 0, 1));
    
    //MARIO
    param_config::update_float_2(*FIGHTER_KIND_MARIO, vec![9].clone(),(hash40("param_special_hi"), hash40("cappy_prob"), 0.0));

    //CLAUS
    param_config::update_float_2(*FIGHTER_KIND_LUCAS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("walk_speed_max"), 0, 1.47));
    param_config::update_float_2(*FIGHTER_KIND_LUCAS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("ground_brake"), 0, 0.045));
    param_config::update_float_2(*FIGHTER_KIND_LUCAS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("dash_speed"), 0, 2.24));
    param_config::update_float_2(*FIGHTER_KIND_LUCAS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("run_speed_max"), 0, 1.9));
    param_config::update_float_2(*FIGHTER_KIND_LUCAS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("air_speed_x_stable"), 0, 1.4883));
    param_config::update_float_2(*FIGHTER_KIND_LUCAS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("dive_speed_y"), 0, 2.35));
    param_config::update_float_2(-*WEAPON_KIND_LUCAS_PK_FIRE, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_pkfire"), hash40("speed_ground"), 7.5));

    //SILVER
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("walk_accel_mul"), 0, 0.15));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("walk_accel_add"), 0, 0.04));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("walk_speed_max"), 0, 1.52));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("ground_brake"), 0, 0.115));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("dash_speed"), 0, 6.91));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("run_accel_mul"), 0, 0.14));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("run_accel_add"), 0, 2.25));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("run_speed_max"), 0, 6.07));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_accel_x_mul"), 0, 0.055));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_speed_x_stable"), 0, 1.1));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_speed_y_stable"), 0, 1.356));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_aerial_y"), 0, 38.0));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("weight"), 0, 82.0));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("shield_radius"), 0, 12.0));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("scale"), 0, 0.865));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_speed_x_mul"), 0, 0.5));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_initial_y"), 0, 19.25));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_y"), 0, 32.0));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("mini_jump_y"), 0, 12.2));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_accel_y"), 0, 0.0842));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_accel_y_stable"), 0, 1.57));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("dive_speed_y"), 0, 2.78));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_n"), 0, 2.0));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_f"), 0, 5.0));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_b"), 0, 4.0));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_hi"), 0, 3.0));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_lw"), 0, 7.0));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_frame"), 0, 3.0));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_heavy_frame"), 0, 4.0));
    param_config::update_int_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("attack_combo_max"), 0, 1));
    param_config::update_int_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_count_max"), 0, 5));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_water_damage"), hash40("water_damage"), 0.0));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_water_damage"), hash40("water_damage_limit"), 0.0));
    param_config::update_float_2(*FIGHTER_KIND_MEWTWO, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_water_damage"), hash40("water_damage_interval"), 0.0));
    param_config::update_float_2(-*WEAPON_KIND_MEWTWO_SHADOWBALL, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_shadowball"), hash40("min_speed"), 4.0));
    param_config::update_float_2(-*WEAPON_KIND_MEWTWO_SHADOWBALL, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_shadowball"), hash40("max_speed"), 4.0));
    param_config::update_float_2(-*WEAPON_KIND_MEWTWO_SHADOWBALL, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_shadowball"), hash40("flicker_scale_min"), 0.0));
    param_config::update_float_2(-*WEAPON_KIND_MEWTWO_SHADOWBALL, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_shadowball"), hash40("flicker_scale_max"), 0.0));
    param_config::update_int_2(-*WEAPON_KIND_MEWTWO_SHADOWBALL, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_shadowball"), hash40("life"), 600));

    //WALUIGI
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("walk_accel_mul"), 0, 0.097));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("walk_accel_add"), 0, 0.243));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("walk_speed_max"), 0, 1.24));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("ground_brake"), 0, 0.062));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("dash_speed"), 0, 3.91));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("run_accel_mul"), 0, 0.12));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("run_accel_add"), 0, 0.026));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("run_speed_max"), 0, 3.65));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_speed_x"), 0, 0.87));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_speed_x_mul"), 0, 0.768));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_speed_x_max"), 0, 2.672));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_aerial_speed_x_mul"), 0, 1.089));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_y"), 0, 47.848));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_aerial_y"), 0, 28.248));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_initial_y"), 0, 26.548));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("mini_jump_y"), 0, 15.155));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_accel_x_mul"), 0, 0.085));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_accel_x_add"), 0, 0.02));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_speed_x_stable"), 0, 1.74));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_speed_y_stable"), 0, 2.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("weight"), 0, 93.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("shield_radius"), 0, 11.8));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("dive_speed_y"), 0, 2.2));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_n"), 0, 8.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_f"), 0, 6.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_b"), 0, 10.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_hi"), 0, 9.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_lw"), 0, 6.0));
    param_config::update_int_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("attack_combo_max"), 0, 2));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_private"), hash40("super_special_damage"), 9999.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_special_hi"), hash40("start_speed_y_mul"), 0.1));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_special_hi"), hash40("stick_x_min"), 0.1));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_special_hi"), hash40("stick_x_max"), 0.1));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_special_hi"), hash40("stick_x_speed_mul_max"), 0.1));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_special_hi"), hash40("control_frame"), 2.0));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_special_hi"), hash40("air_speed_y_mul"), 1.676));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_special_hi"), hash40("speed_y_mul"), 1.55));
    param_config::update_float_2(*FIGHTER_KIND_DOLLY, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_special_hi"), hash40("lr_stick_x"), 0.2));

    //BLAZIKEN
    param_config::update_int_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("attack_combo_max"), 0, 3));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("walk_accel_mul"), 0, 0.161));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("walk_accel_add"), 0, 0.188));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("walk_speed_max"), 0, 1.34));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("ground_brake"), 0, 0.0894));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("dash_speed"), 0, 3.074));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("run_accel_mul"), 0, 0.096));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("run_accel_add"), 0, 0.014));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("run_speed_max"), 0, 2.757));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_speed_x"), 0, 0.8));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_speed_x_mul"), 0, 0.721));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_speed_x_max"), 0, 1.568));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_aerial_speed_x_mul"), 0, 0.085));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_y"), 0, 41.32));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_aerial_y"), 0, 36.448));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_initial_y"), 0, 21.5));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("mini_jump_y"), 0, 18.653));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_accel_x_mul"), 0, 0.0917));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_accel_x_add"), 0, 0.04));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_speed_x_stable"), 0, 1.57));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("weight"), 0, 90.0));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("scale"), 0, 1.0));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("shield_radius"), 0, 12.2));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("dive_speed_y"), 0, 2.756));
    param_config::update_int_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("wall_jump_type"), 0, 1));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_n"), 0, 3.0));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_f"), 0, 5.0));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_b"), 0, 6.0));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_hi"), 0, 4.0));
    param_config::update_float_2(*FIGHTER_KIND_CAPTAIN, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_lw"), 0, 9.0));

    //KRYSTAL
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("jump_count_max"), 0, 4));
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("wall_jump_type"), 0, 1));
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("attack_combo_max"), 0, 3));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("shield_radius"), 0, 13.96));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("walk_speed_max"), 0, 1.65));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("ground_brake"), 0, 0.115));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("run_speed_max"), 0, 3.66));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("jump_speed_x_max"), 0, 1.9));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("jump_initial_y"), 0, 20.55));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("jump_y"), 0, 37.9));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("mini_jump_y"), 0, 18.2));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("jump_aerial_y"), 0, 55.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("air_accel_x_mul"), 0, 0.12));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("air_speed_x_stable"), 0, 1.74));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("air_speed_y_stable"), 0, 2.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("air_break_x"), 0, 0.015));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("air_break_y"), 0, 0.015));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("dive_speed_y"), 0, 3.24));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("air_accel_y"), 0, 0.19));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("weight"), 0, 76.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("fly_speed_y_mul"), 0, 2.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("landing_attack_air_frame_n"), 0, 5.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("landing_attack_air_frame_f"), 0, 3.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("landing_attack_air_frame_b"), 0, 6.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("landing_attack_air_frame_hi"), 0, 4.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("landing_attack_air_frame_lw"), 0, 6.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("landing_frame"), 0, 3.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("height"), 0, 14.2));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("speed_y_table"), 1, 2.05));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("speed_y_table"), 2, 2.1));
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_n"), hash40("charge_frame"), 0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_n"), hash40("turn_stick_x"), 0.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_n"), hash40("speed_x_mul"), 0.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_n"), hash40("up_stick_y"), 0.0));
    param_config::update_float_2(-*WEAPON_KIND_PITB_BOWARROW, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_bowarrow"), hash40("angle_up"), 0.0));
    param_config::update_float_2(-*WEAPON_KIND_PITB_BOWARROW, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_bowarrow"), hash40("control_angle"), 0.0));
    param_config::update_float_2(-*WEAPON_KIND_PITB_BOWARROW, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_bowarrow"), hash40("power"), 32.0));
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_hi"), hash40("stop_y_frame"), 8));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_hi"), hash40("accel_y"), 0.04));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_hi"), hash40("rush_speed"), 4.6));
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_hi"), hash40("landing_frame"), 20));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_hi"), hash40("rush_angle"), 60.0));
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_lw"), hash40("frame_min"), 28));
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_lw"), hash40("frame_max"), 900));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_lw"), hash40("shield_max"), 3000.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_lw"), hash40("shield_recovery"), 4.45));
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_lw"), hash40("no_shield_frame"), 240));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_lw"), hash40("life_mul"), 1.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_lw"), hash40("end_scale"), 1.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_lw"), hash40("end_offset_z"), 0.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_lw"), hash40("end_offset_y"), 0.0));
    param_config::update_float_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_special_lw"), hash40("reflector_max"), 3000.0));
    param_config::update_int_2(*FIGHTER_KIND_PITB, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_glide"), hash40("jump_button_hold_glide_frame"), 9999));

    //NINTEN
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("walk_accel_mul"), 0, 0.071));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("walk_accel_add"), 0, 0.105));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("walk_speed_max"), 0, 1.287));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("ground_brake"), 0, 0.112));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("dash_speed"), 0, 2.41));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("run_accel_mul"), 0, 0.106));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("run_accel_add"), 0, 0.085));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("run_speed_max"), 0, 2.154));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_aerial_speed_x_mul"), 0, 0.9957));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_y"), 0, 32.8));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_aerial_y"), 0, 60.7));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("jump_initial_y"), 0, 17.5));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("mini_jump_y"), 0, 14.189));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_accel_x_mul"), 0, 0.092));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_accel_x_add"), 0, 0.014));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_speed_x_stable"), 0, 1.363));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("air_speed_y_stable"), 0, 1.25));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("weight"), 0, 82.0));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("scale"), 0, 0.9));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("shield_radius"), 0, 11.3));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("dive_speed_y"), 0, 2.2));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_n"), 0, 4.0));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_f"), 0, 6.0));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_b"), 0, 4.0));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_hi"), 0, 3.0));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("landing_attack_air_frame_lw"), 0, 6.0));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FLASH, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkflash"), hash40("power_min"), 15.0));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FLASH, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkflash"), hash40("power_mul"), 0.7));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FLASH, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkflash"), hash40("count"), 240.0));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FLASH, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkflash"), hash40("size_min"), 1.0));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FLASH, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkflash"), hash40("size_max"), 3.75));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FLASH, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkflash"), hash40("angle"), -30.0));
    param_config::update_int_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkfire"), hash40("life"), 2000));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkfire"), hash40("angle_ground"), 0.0));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkfire"), hash40("angle_air"), -25.0));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkfire"), hash40("speed_ground"), 3.0));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkfire"), hash40("speed_air"), 3.4));
    param_config::update_int_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkfire"), hash40("pillar_life"), 10));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkfire"), hash40("pillar_damage_mul"), 4.0));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkfire"), hash40("pillar_scale_min"), 1.0));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkfire"), hash40("pillar_offset_y"), 0.0));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkfire"), hash40("pillar_accel_y"), 0.0));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_pkfire"), hash40("pillar_speed_max_y"), 0.0));
    param_config::update_int_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_special_lw"), hash40("stop_y_time"), 4));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_special_lw"), hash40("accel_y"), 0.24));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_special_lw"), hash40("absorb_power_max"), 7000.0));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![120,121,122,123,124,125,126,127].clone(),(hash40("param_special_lw"), hash40("phy_magnet_effect_scale"), 1.2));

    //KAMEK
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("walk_accel_mul"), 0, 0.065));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("walk_accel_add"), 0, 0.12));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("walk_speed_max"), 0, 1.153));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("jump_initial_y"), 0, 16.5));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("mini_jump_y"), 0, 15.0));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("jump_y"), 0, 35.24));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("jump_aerial_y"), 0, 66.0));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("dash_speed"), 0, 4.124));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("run_accel_mul"), 0, 0.115));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("run_accel_add"), 0, 0.073));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("run_speed_max"), 0, 3.96));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("ground_brake"), 0, 0.0694));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("air_accel_x_mul"), 0, 0.082));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("air_accel_x_add"), 0, 0.011));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("air_speed_x_stable"), 0, 1.357));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("air_speed_y_stable"), 0, 1.18));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("air_accel_y"), 0, 0.071));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("dive_speed_y"), 0, 2.43));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("weight"), 0, 91.0));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("shield_radius"), 0, 12.8));
    param_config::update_int_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("attack_combo_max"), 0, 1));
    param_config::update_int_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("jump_count_max"), 0, 5));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("landing_attack_air_frame_n"), 0, 2.0));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("landing_attack_air_frame_f"), 0, 5.0));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("landing_attack_air_frame_b"), 0, 5.0));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("landing_attack_air_frame_hi"), 0, 6.0));
    param_config::update_float_2(*FIGHTER_KIND_NESS, vec![64,65,66,67,68,69,70,71].clone(),(hash40("landing_attack_air_frame_lw"), 0, 8.0));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_pkfire"), hash40("speed_ground"), 3.0));
    param_config::update_float_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_pkfire"), hash40("speed_air"), 5.0));
    param_config::update_int_2(-*WEAPON_KIND_NESS_PK_FIRE, vec![64,65,66,67,68,69,70,71].clone(),(hash40("param_pkfire"), hash40("life"), 300));
}

pub fn install() {
    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, new_fighter_params);
    }
}