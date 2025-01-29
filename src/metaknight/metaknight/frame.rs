use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_metaknight_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    let prev_status_kind = StatusModule::prev_status_kind(fighter.module_accessor, 0);
    FighterSpecializer_MetaKnight::meta_power(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_GLIDE {
        let mut angle = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_ANGLE);
        let angle_se_pitch_ratio = WorkModule::get_param_float(fighter.module_accessor, hash40("param_glide"), hash40("angle_se_pitch_ratio"));
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        SoundModule::set_se_pitch_ratio(fighter.module_accessor, Hash40::new("se_metaknight_glide_loop"), 1.0 + angle * angle_se_pitch_ratio);
        if [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_HI_LOOP].contains(&prev_status_kind) {
            let glide_frame = WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_GLIDE_WORK_FLOAT_GLIDE_FRAME);
            if glide_frame < 60.0 {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
            }
            else {
                WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_DEAD);
            }
        }
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
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::heal(fighter.module_accessor, -1.2, 0);
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_recovery"), Hash40::new("top"), &VECTOR_ZERO, &VECTOR_ZERO, 1.0, true, 0, 0, 0, 0, 0, true, true);
            if DamageModule::damage(fighter.module_accessor, 0) > 0.0 {
                SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_lifeup"), true, false, false, false, enSEType(0));
            }
        }
    };
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_SPECIAL_LW) {
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) { 
            fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_LW.into(), false.into());
        }
    }
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) 
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        fighter.change_status(FIGHTER_STATUS_KIND_SPECIAL_GUARD.into(), true.into());
    }
}

pub fn install() {
    Agent::new("metaknight")
    .on_line(Main, frame_metaknight_Main)
    .install();
}