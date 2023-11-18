use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_METAKNIGHT )]
fn frame_metaknight(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let params = GlideParams::get(fighter);
        let lr = PostureModule::lr(fighter.module_accessor);
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:1.1, y:1.1, z:1.1});
        //Inner Meta Mechanic
        FighterSpecializer_MetaKnight::meta_power(fighter);
        //SFX Controllers
        if [
            *FIGHTER_STATUS_KIND_LANDING, *FIGHTER_STATUS_KIND_LANDING_LIGHT, *FIGHTER_STATUS_KIND_GLIDE_LANDING,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_MISS_FOOT,
            *FIGHTER_STATUS_KIND_DAMAGE, *FIGHTER_STATUS_KIND_DAMAGE_AIR, *FIGHTER_STATUS_KIND_DAMAGE_FALL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY, *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR,
            *FIGHTER_STATUS_KIND_CLIFF_CATCH, *FIGHTER_STATUS_KIND_GLIDE_ATTACK, *FIGHTER_STATUS_KIND_GLIDE_END
        ].contains(&status_kind) { 
            STOP_SE(fighter, Hash40::new("se_metaknight_glide_start"));
            STOP_SE(fighter, Hash40::new("se_metaknight_glide_loop"));
            STOP_SE(fighter, Hash40::new("se_metaknight_special_h02"));
        };
        //Specials
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_N_SPIN {
            fighter.sub_air_check_fall_common();
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CLIFF_CATCH);
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_AIR_HI_START].contains(&status_kind) {
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH {
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -1.0, 0);
            }
        };
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if frame > 10.0 {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) || 
                ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END.into(), true.into());
                }
                else if fighter.global_table[PAD_FLAG].get_i32() & *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER != 0 {
                    fighter.change_status(FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK.into(), true.into());
                }
            }
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_ATTACK {
            special_lw_attack_function(fighter);
        }
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_LW_END {
            special_lw_end_function(fighter);
        };
        /*if status_kind == *FIGHTER_STATUS_KIND_ESCAPE_AIR && 
        ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_AIR_DIVE, false);
        }*/
    }
}

unsafe fn special_lw_attack_function(fighter: &mut L2CFighterCommon) {
    let frame = MotionModule::frame(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        if frame > 60.0 {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) || AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_S);
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_METAKNIGHT_INSTANCE_WORK_ID_DISABLE_AIR_SPECIAL_LW);
        if frame > 20.0 {
            CancelModule::enable_cancel(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 3.0);
        }
    }
}

unsafe fn special_lw_end_function(fighter: &mut L2CFighterCommon) {
    let frame = MotionModule::frame(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
        if motion_kind == hash40("special_air_lw_end") && frame >= 33.0 {      
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_metaknight
    );
}