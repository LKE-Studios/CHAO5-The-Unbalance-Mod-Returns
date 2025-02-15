use crate::imports::BuildImports::*;

pub static bounce_y_speed : f32 = 1.4;
pub static bounce_count : i32 = 6;

pub unsafe extern "C" fn frame_funky_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let FUNKY = color >= 120 && color <= 127; 
    if FUNKY {
        funky_update(fighter);
        let scale = WorkModule::get_param_float(fighter.module_accessor, hash40("scale"), 0);
        if ModelModule::scale(fighter.module_accessor) == scale {
            ModelModule::set_scale(fighter.module_accessor, 1.0);
            AttackModule::set_attack_scale(fighter.module_accessor, 1.0, true);
            GrabModule::set_size_mul(fighter.module_accessor, 1.0);
        };
        if status_kind == *FIGHTER_STATUS_KIND_DAMAGE_AIR || status_kind == *FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("pzenigame_takinobori_splash"), false, false);
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("pzenigame_takinobori"), false, false);
        }  
        if motion_kind == hash40("attack_air_lw") && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLAG_CHECK_BOUNCE) {
            if frame >= 29.0 {
                CancelModule::enable_cancel(fighter.module_accessor);
            }
        };
        if motion_kind != hash40("attack_air_lw") {
            WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLAG_BOUNCE);
        }
        let bounce_count_int = WorkModule::get_int(fighter.module_accessor, *FIGHTER_FUNKY_INSTANCE_WORK_ID_INT_BOUNCE_COUNT);
        if bounce_count_int > bounce_count {
            if motion_kind == hash40("attack_air_lw") {
                if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
                && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLAG_BOUNCE) {
                    WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLAG_CHECK_BOUNCE);
                    WorkModule::add_int(fighter.module_accessor, 1, *FIGHTER_FUNKY_INSTANCE_WORK_ID_INT_BOUNCE_COUNT);
                    let lr = PostureModule::lr(fighter.module_accessor);
                    let sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * lr;
                    sv_kinetic_energy!(set_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_STOP, bounce_y_speed, 0.0);
                    WorkModule::set_flag(fighter.module_accessor, false, *FIGHTER_FUNKY_INSTANCE_WORK_ID_FLAG_BOUNCE);
                }
            }
        }
        if situation_kind != *SITUATION_KIND_AIR {
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_FUNKY_INSTANCE_WORK_ID_INT_BOUNCE_COUNT);
        };
        if status_kind == *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_CANCEL {
            if situation_kind == *SITUATION_KIND_AIR {
                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE) == *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_AIR_ESCAPE_AIR {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_DONKEY_SPECIAL_N_CANCEL_TYPE_NONE, *FIGHTER_DONKEY_STATUS_SPECIAL_N_WORK_INT_CANCEL_TYPE);
                }
            }
        }
        if status_kind == *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_LOOP {
            let charge_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_DONKEY_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
            let max_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
            if charge_frame == max_charge_frame {
                SoundModule::play_se(fighter.module_accessor, Hash40::new("se_donkey_special_n07"), true, false, false, false, enSEType(0));
            }
        }
        if status_kind == *FIGHTER_DONKEY_STATUS_KIND_SPECIAL_N_ATTACK {
            fighter.sub_wait_ground_check_common(false.into());
            fighter.sub_air_check_fall_common();
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DONKEY_STATUS_SPECIAL_N_FLAG_MAX) {
                let lr = PostureModule::lr(fighter.module_accessor);
                let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
                let stick_y = ControlModule::get_stick_y(fighter.module_accessor);
                let mut stick_direction = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_STICK_DIRECTION);
                stick_direction = ControlModule::get_stick_dir(fighter.module_accessor) * (180.0 / PI);
                if stick_x >= -0.2 && stick_x <= 0.2 && stick_y >= -0.2 && stick_y <= 0.2 {
                    WorkModule::set_float(fighter.module_accessor, 361.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_STICK_DIRECTION);
                } 
                else if stick_direction <= -67.5 {
                    WorkModule::mul_float(fighter.module_accessor, -1.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_STICK_DIRECTION);
                }
                if stick_direction >= -67.5 && stick_direction < -20.0 && stick_x * lr > 0.0 { //(Angled Down)
                    param_config::update_float(-*WEAPON_KIND_LUIGI_FIREBALL, vec![120,121,122,123,124,125,126,127], (hash40("param_boot"),hash40("angle")), -25.0);
                }
                else if stick_direction > 20.0 && stick_direction <= 67.5 && stick_x * lr > 0.0 { //(Angled Up)
                    param_config::update_float(-*WEAPON_KIND_LUIGI_FIREBALL, vec![120,121,122,123,124,125,126,127], (hash40("param_boot"),hash40("angle_air")), 30.0);
                }
            }
        }
        if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind)
        && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            fighter.change_status(FIGHTER_FUNKY_STATUS_KIND_SPECIAL_HI_C2.into(), true.into());
        }  
    }
}     

unsafe fn barrel_air_despawn(fighter: &mut L2CFighterCommon, module_accessor: *mut smash::app::BattleObjectModuleAccessor, status_kind: i32, motion_kind: u64) {
    let launch = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_FUNKY_STATUS_SPECIAL_HI_FLAG_LAUNCH);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI && !launch {
        return;
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL) {
        let barrel_modules = CustomModule::get_article_module_accessor(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL);
        let barrel_frame = MotionModule::frame(barrel_modules);
        if barrel_frame > 40.0 {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_DONKEY_GENERATE_ARTICLE_DKBARREL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        }
        else if barrel_frame > 30.0 {
            if (barrel_frame.floor() % 2.0 == 0.0) {
                ModelModule::set_alpha(barrel_modules, 0.5);
            }
            else {
                ModelModule::set_alpha(barrel_modules, 1.0);
            }
        }
    }
}

unsafe fn funky_update(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let module_accessor = fighter.module_accessor;
    barrel_air_despawn(fighter, module_accessor, status_kind, motion_kind);
}

pub fn install() {
    Agent::new("donkey")
    .on_line(Main, frame_funky_Main)
    .install();
}