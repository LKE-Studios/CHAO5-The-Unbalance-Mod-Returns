use smash::lib::lua_const::*;
use smash::app::*;
use smash::hash40;
use smash::app::lua_bind::*;
use crate::utils::*;

static mut FLOAT_OFFSET : usize = 0x4e53c0;
static mut INT_OFFSET : usize = 0x4e5380;

#[skyline::hook(offset=FLOAT_OFFSET)] //"Custom character" exclusive fighter attributes for FLOAT
pub unsafe fn get_param_float(boma: u64, param_type: u64, param_hash: u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let ENTRY_ID = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let weapon_color = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let boma_reference = &mut *module_accessor;
    let CLAUS = color >= 64 && color <= 71;
    let KNUCKLES = color >= 120 && color <= 127;
    let MIDBUS = color >= 120 && color <= 127;
    let SILVER = color >= 120 && color <= 127;
    let WALUIGI = color >= 120 && color <= 130;
    let WEAPON_CLAUS = weapon_color >= 64 && weapon_color <= 71;
    let WEAPON_SILVER = weapon_color >= 120 && weapon_color <= 127;
    if boma_reference.is_fighter() {
        if fighter_kind == FIGHTER_KIND_LUCAS && CLAUS {
            if param_hash == 0 {
                if param_type == hash40("walk_speed_max") {
                    return 1.47; //Lucas 1.65
                }
                if param_type == hash40("ground_brake") {
                    return 0.045; //Lucas 0.116
                }
                if param_type == hash40("dash_speed") {
                    return 2.24; //Lucas 2.15
                }
                if param_type == hash40("run_speed_max") {
                    return 1.9; //Lucas 1.824
                }
                if param_type == hash40("air_speed_x_stable") {
                    return 1.4883; //Lucas 1.454
                }
                if param_type == hash40("dive_speed_y") {
                    return 2.35; //Lucas 2.242
                }
            }
        }
        if fighter_kind == FIGHTER_KIND_KOOPA && MIDBUS {
            if param_hash == 0 {
                if param_type == hash40("walk_speed_max") {
                    return 1.64; //Bowser 1.792
                }
                if param_type == hash40("dash_speed") {
                    return 3.15; //Bowser 2.955
                }
                if param_type == hash40("run_speed_max") {
                    return 3.073; //Bowser 2.7761
                }
                if param_type == hash40("air_speed_x_stable") {
                    return 1.756; //Bowser 1.9278
                }
                if param_type == hash40("dive_speed_y") {
                    return 3.043; //Bowser 9999.9
                }
                if param_type == hash40("air_speed_y_stable") {
                    return 1.91; //Bowser 1.865
                }
                if param_type == hash40("weight") {
                    return 166.0; //Bowser 180.0
                }
                if param_type == hash40("scale") {
                    return 0.8; //Bowser 0.91
                }
            }
        }
        if fighter_kind == FIGHTER_KIND_MARIO {
            if color == 9 {
                if param_type == hash40("param_special_hi") {
                    if param_hash == hash40("cappy_prob") {
                        return 0.0; //Other Mario costumes 90.0
                    }
                }
            }
        }
        if fighter_kind == FIGHTER_KIND_NESS {
            if color >= 8 && color <= 11 {
                if param_hash == 0 {
                    if param_type == hash40("weight") {
                        return 82.0;
                    }
                }
            }
        }
        if fighter_kind == FIGHTER_KIND_SONIC && KNUCKLES {
            if param_hash == 0 {
                if param_type == hash40("walk_accel_mul") {
                    return 0.15;
                }
                if param_type == hash40("walk_accel_add") {
                    return 0.04;
                }
                if param_type == hash40("walk_speed_max") {
                    return 1.7;
                }
                if param_type == hash40("ground_brake") {
                    return 0.07;
                }
                if param_type == hash40("dash_speed") {
                    return 5.45; //Sonic 11.165
                }
                if param_type == hash40("run_accel_mul") {
                    return 0.12;
                }
                if param_type == hash40("run_accel_add") {
                    return 2.3; //Sonic 2.944
                }
                if param_type == hash40("run_speed_max") {
                    return 6.0; //Sonic 10.85
                }
                if param_type == hash40("jump_speed_x") {
                    return 0.72;
                }
                if param_type == hash40("jump_speed_x_mul") {
                    return 0.8;
                }
                if param_type == hash40("jump_speed_x_max") {
                    return 2.5;
                }
                if param_type == hash40("jump_aerial_speed_x_mul") {
                    return 1.0;
                }
                if param_type == hash40("air_accel_x_mul") {
                    return 0.045;
                }
                if param_type == hash40("air_accel_x_add") {
                    return 0.1; 
                }
                if param_type == hash40("air_speed_x_stable") {
                    return 1.687;
                }
                if param_type == hash40("dive_speed_y") {
                    return 2.88;
                }
                if param_type == hash40("weight") {
                    return 94.0;
                }
                if param_type == hash40("scale") {
                    return 0.966; 
                }
                if param_type == hash40("shield_radius") {
                    return 12.8;
                }
                if param_type == hash40("landing_attack_air_frame_n") {
                    return 2.0; 
                }
                if param_type == hash40("landing_attack_air_frame_f") {
                    return 4.0;
                }
                if param_type == hash40("landing_attack_air_frame_b") {
                    return 3.0; 
                }
                if param_type == hash40("landing_attack_air_frame_hi") {
                    return 2.0;
                }
                if param_type == hash40("landing_attack_air_frame_lw") {
                    return 5.0;
                }
                if param_type == hash40("landing_frame") {
                    return 3.0;
                }
                if param_type == hash40("landing_heavy_frame") {
                    return 4.0; //Sonic 3.0
                }
            }
            if param_type == hash40("param_special_hi") && param_hash == hash40("special_hi_jump_speed_y") {
                return 10.2;
            }
            if param_type == hash40("param_water_damage") && param_hash == hash40("water_damage_interval") {
                return 0.0;
            }
            if param_type == hash40("param_water_damage") && param_hash == hash40("water_damage") {
                return 0.0;
            }
            if param_type == hash40("param_water_damage") && param_hash == hash40("water_damage_limit") {
                return 0.0;
            }
        }
        if fighter_kind == FIGHTER_KIND_MEWTWO && SILVER {
            if param_hash == 0 {
                if param_type == hash40("walk_accel_mul") {
                    return 0.15;
                }
                if param_type == hash40("walk_accel_add") {
                    return 0.04;
                }
                if param_type == hash40("walk_speed_max") {
                    return 1.52;
                }
                if param_type == hash40("ground_brake") {
                    return 0.084;
                }
                if param_type == hash40("dash_speed") {
                    return 6.91;
                }
                if param_type == hash40("run_accel_mul") {
                    return 0.14;
                }
                if param_type == hash40("run_accel_add") {
                    return 2.25; 
                }
                if param_type == hash40("run_speed_max") {
                    return 6.07; 
                }
                if param_type == hash40("air_accel_x_mul") {
                    return 0.055;
                }
                if param_type == hash40("air_accel_x_add") {
                    return 0.1;
                }
                if param_type == hash40("air_speed_x_stable") {
                    return 1.356;
                }
                if param_type == hash40("jump_aerial_y") {
                    return 38.0;
                }
                if param_type == hash40("weight") {
                    return 82.0;
                }
                if param_type == hash40("shield_radius") {
                    return 12.0;
                }
                if param_type == hash40("scale") {
                    return 0.865; 
                }
                if param_type == hash40("jump_speed_x_mul") {
                    return 0.5;
                }
                if param_type == hash40("jump_initial_y") {
                    return 19.25;
                }
                if param_type == hash40("jump_y") {
                    return 32.0;
                }
                if param_type == hash40("air_accel_y") {
                    return 0.0842;
                }
                if param_type == hash40("air_speed_y_stable") {
                    return 1.57;
                }
                if param_type == hash40("dive_speed_y") {
                    return 2.78;
                }
                if param_type == hash40("landing_attack_air_frame_f") {
                    return 5.0;
                }
                if param_type == hash40("landing_attack_air_frame_n") {
                    return 2.0;
                }
                if param_type == hash40("landing_attack_air_frame_b") {
                    return 4.0;
                }
                if param_type == hash40("landing_attack_air_frame_hi") {
                    return 3.0; 
                }
                if param_type == hash40("landing_attack_air_frame_lw") {
                    return 7.0;
                }
                if param_type == hash40("landing_frame") {
                    return 3.0; 
                }
                if param_type == hash40("mini_jump_y") {
                    return 12.2;
                }
            }
            if param_type == hash40("param_water_damage") && param_hash == hash40("water_damage_interval") {
                return 0.0;
            }
            if param_type == hash40("param_water_damage") && param_hash == hash40("water_damage") {
                return 0.0;
            }
            if param_type == hash40("param_water_damage") && param_hash == hash40("water_damage_limit") {
                return 0.0;
            }
        }
        if fighter_kind == FIGHTER_KIND_DOLLY && WALUIGI {
            if param_hash == 0 {
                if param_type == hash40("walk_accel_mul") {
                    return 0.097;
                }
                if param_type == hash40("walk_accel_add") {
                    return 0.243;
                }
                if param_type == hash40("walk_speed_max") {
                    return 0.84;
                }
                if param_type == hash40("ground_brake") {
                    return 0.062;
                }
                if param_type == hash40("dash_speed") {
                    return 3.91;
                }
                if param_type == hash40("run_accel_mul") {
                    return 0.12;
                }
                if param_type == hash40("run_accel_add") {
                    return 0.026;
                }
                if param_type == hash40("run_speed_max") {
                    return 3.65;
                }
                if param_type == hash40("jump_speed_x") {
                    return 0.87;
                }
                if param_type == hash40("jump_speed_x_mul") {
                    return 0.768;
                }
                if param_type == hash40("jump_speed_x_max") {
                    return 2.672;
                }
                if param_type == hash40("jump_aerial_speed_x_mul") {
                    return 1.089;
                }
                if param_type == hash40("jump_y") {
                    return 47.848;
                }
                if param_type == hash40("air_accel_y") {
                    return 0.09;
                }
                if param_type == hash40("jump_aerial_y") {
                    return 28.248;
                }
                if param_type == hash40("jump_initial_y") {
                    return 28.548;
                }
                if param_type == hash40("mini_jump_y") {
                    return 15.155;
                }
                if param_type == hash40("air_accel_x_mul") {
                    return 0.085;
                }
                if param_type == hash40("air_accel_x_add") {
                    return 0.02;
                }
                if param_type == hash40("dive_speed_y") {
                    return 2.2;
                }
                if param_type == hash40("weight") {
                    return 93.0;
                }
                if param_type == hash40("shield_radius") {
                    return 11.8;
                }
                if param_type == hash40("landing_attack_air_frame_n") {
                    return 8.0;
                }
                if param_type == hash40("landing_attack_air_frame_f") {
                    return 6.0;
                }
                if param_type == hash40("landing_attack_air_frame_b") {
                    return 10.0;
                }
                if param_type == hash40("landing_attack_air_frame_hi") {
                    return 9.0;
                }
                if param_type == hash40("landing_attack_air_frame_lw") {
                    return 6.0;
                }
            }
            if param_type == hash40("param_private") && param_hash == hash40("super_special_damage") {
                return 9999.0;
            }
            if param_type == hash40("param_special_hi") && param_hash == hash40("start_speed_y_mul") {
                return 0.1;
            }
            if param_type == hash40("param_special_hi") && param_hash == hash40("stick_x_min") {
                return 0.1;
            }
            if param_type == hash40("param_special_hi") && param_hash == hash40("stick_x_max") {
                return 0.1;
            }
            if param_type == hash40("param_special_hi") && param_hash == hash40("stick_x_speed_mul_max") {
                return 0.1;
            }
            if param_type == hash40("param_special_hi") && param_hash == hash40("control_frame") {
                return 2.0;
            } 
            if param_type == hash40("param_special_hi") && param_hash == hash40("air_speed_y_mul") {
                return 1.676;
            }
            if param_type == hash40("param_special_hi") && param_hash == hash40("speed_y_mul") {
                return 1.55;
            }		
            if param_type == hash40("param_special_hi") && param_hash == hash40("lr_stick_x") {
                return 0.2;
            }
        }
    }
    else if boma_reference.is_weapon() {
        if fighter_kind == *WEAPON_KIND_MEWTWO_SHADOWBALL && WEAPON_SILVER {
            if param_type == hash40("param_shadowball") {
                /*if param_hash == hash40("angle") {
                    if SPECIAL_N_ANGLE[ENTRY_ID] == -1.0 && SPECIAL_N_GET_ANGLE[ENTRY_ID] && StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR {
                        return -30.0;
                    }
                    else {
                        return 0.0;
                    }
                }*/
                if param_hash == hash40("min_speed") {
                    return 4.0;
                }
                if param_hash == hash40("max_speed") {
                    return 4.0;
                }
                if param_hash == hash40("flicker_scale_min") {
                    return 0.0;
                }
                if param_hash == hash40("flicker_scale_max") {
                    return 0.0;
                }
            }
        }
        if fighter_kind == *WEAPON_KIND_LUCAS_PK_FIRE && WEAPON_CLAUS {
            if param_type == hash40("param_pkfire") {
                if param_hash == hash40("speed_ground") {
                    return 7.5; //Lucas 3.3
                }
            }
        }
    }
    ret
}

#[skyline::hook(offset=INT_OFFSET)] //"Custom character" exclusive fighter attributes for INT
pub unsafe fn get_param_int(boma: u64, param_type: u64, param_hash: u64) -> i32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let weapon_color = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let boma_reference = &mut *module_accessor;
    let SILVER = color >= 120 && color <= 127;
    let WALUIGI = color >= 120 && color <= 130;
    let WEAPON_SILVER = weapon_color >= 120 && weapon_color <= 127;
    if boma_reference.is_fighter() {
        if fighter_kind == FIGHTER_KIND_MEWTWO && SILVER {
            if param_hash == 0 {
                if param_type == hash40("landing_heavy_frame") {
                    return 4; 
                }
                if param_type == hash40("attack_combo_max") {
                    return 1;
                }
                if param_type == hash40("jump_count_max") {
                    return 5;
                }
            }
        }
        if fighter_kind == FIGHTER_KIND_DOLLY && WALUIGI {
            if param_hash == 0 {
                if param_type == hash40("attack_combo_max") {
                    return 2;
                }
            }
        }
    }
    else if boma_reference.is_weapon() {
        if fighter_kind == *WEAPON_KIND_MEWTWO_SHADOWBALL && WEAPON_SILVER {
            if param_type == hash40("param_shadowball") {
                if param_hash == hash40("life") {
                    return 600;
                }
            }
        }
    }
    ret
}

pub fn install() {
    skyline::install_hooks!(
        //get_param_float,
        //get_param_int
    );
}