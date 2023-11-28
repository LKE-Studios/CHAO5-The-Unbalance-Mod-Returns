use crate::imports::BuildImports::*;

#[fighter_frame( agent = FIGHTER_KIND_METAKNIGHT )]
fn frame_metaknight(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
        let frame = MotionModule::frame(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);
        ModelModule::set_joint_scale(fighter.module_accessor, Hash40::new("haver"), &Vector3f{x:1.05, y:1.05, z:1.05});
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
        if status_kind == *FIGHTER_METAKNIGHT_STATUS_KIND_SPECIAL_S_RUSH {
            KineticModule::clear_speed_energy_id(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND);
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -1.0, 0);
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_metaknight
    );
}