use smash::lib::lua_const::*;
use smash::app::*;
use smash::hash40;
use smash::app::lua_bind::*;
use crate::utils::*;

static mut FLOAT_OFFSET : usize = 0x4e53c0;
//static mut INT_OFFSET : usize = 0x4ded80;

#[skyline::hook(offset=FLOAT_OFFSET)] //"Custom character" exclusive fighter attributes
pub unsafe fn get_param_float(boma: u64, param_type: u64, param_hash: u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let weapon_color = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let boma_reference = &mut *module_accessor;
    let CLAUS = color >= 8 && color <= 15;
    let KNUCKLES = color >= 8 && color <= 15;
    let MIDBUS = color >= 8 && color <= 15;
    
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
                    return 1.818; //Lucas 1.724
                }
                if param_type == hash40("dive_speed_y") {
                    return 2.35; //Lucas 2.242
                }
            }
        }
    }
    else if boma_reference.is_weapon() {
        if fighter_kind == *WEAPON_KIND_LUCAS_PK_FIRE && weapon_color >= 8 && weapon_color <= 15 {
            if param_type == hash40("param_pkfire") {
                if param_hash == hash40("speed_ground") {
                    return 7.5; //Lucas 3.3
                }
            }
        }
    }
    if boma_reference.is_fighter() {
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
                    return 0.1; //Sonic 0.12
                }
                if param_type == hash40("air_speed_x_stable") {
                    return 1.687; //Sonic 1.8164
                }
                if param_type == hash40("dive_speed_y") {
                    return 2.88;
                }
                if param_type == hash40("weight") {
                    return 94.0; //Sonic 0.86
                }
                if param_type == hash40("scale") {
                    return 0.966; //Sonic 0.84
                }
                if param_type == hash40("shield_radius") {
                    return 12.8;
                }
                if param_type == hash40("landing_attack_air_frame_n") {
                    return 3.0; //Sonic 1.0
                }
                if param_type == hash40("landing_attack_air_frame_f") {
                    return 3.0; //Sonic 1.0
                }
                if param_type == hash40("landing_attack_air_frame_b") {
                    return 3.0; //Sonic 1.0
                }
                if param_type == hash40("landing_attack_air_frame_hi") {
                    return 2.0; //Sonic 1.0
                }
                if param_type == hash40("landing_attack_air_frame_lw") {
                    return 5.0; //Sonic 3.0
                }
                if param_type == hash40("landing_frame") {
                    return 3.0; //Sonic 2.0
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
    }
ret
}

pub fn install() {
    skyline::install_hooks!(
        get_param_float
    );
}