use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_metaknight_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    //Inner Meta Mechanic
    FighterSpecializer_MetaKnight::meta_power(fighter);
    if status_kind == *FIGHTER_STATUS_KIND_GLIDE {
        let mut angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
        let angle_se_pitch_ratio = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_se_pitch_ratio"));
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_metaknight_glide_loop"), 1.0 + angle * angle_se_pitch_ratio);
    }
    if ![*FIGHTER_STATUS_KIND_GLIDE_START, *FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_KIND_SPECIAL_HI, 
        *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP].contains(&status_kind) { 
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_metaknight_glide_loop"), 0);
        SoundModule::stop_se(fighter.module_accessor, Hash40::new("se_metaknight_glide_start"), 0);
    };
    if status_kind == *FIGHTER_STATUS_KIND_ATTACK_S3 {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH)
        && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_CATCH) {
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH.into(), false.into());
        }
    }
    //Specials
    if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH {
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -1.0, 0);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_recovery"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, true);
            if DamageModule::damage(fighter.module_accessor, 0) > 0.0 {
                PLAY_SE_REMAIN(fighter, Hash40::new("se_common_lifeup"));
            }
        }
    };
    if WorkModule::is_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_FLAG_ENABLE_SPECIAL_LW) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { 
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        }
    }
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) 
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_GUARD.into(), true.into());
    }
}

unsafe extern "C" fn frame_metaknight_Exec(fighter: &mut L2CFighterCommon) {
    ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x: 1.05, y: 1.05, z: 1.05});
}

pub fn install() {
    Agent::new("metaknight")
    .on_line(Main, frame_metaknight_Main)
    .on_line(Exec, frame_metaknight_Exec)
    .install();
}