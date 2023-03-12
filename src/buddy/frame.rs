//use smash::app::sv_animcmd::*;
use smash::lib::lua_const::*;
use smash::phx::Hash40;
use smash::app::lua_bind::*;
use smashline::*;
use smash_script::*;
use smash::lua2cpp::L2CFighterCommon;

#[fighter_frame( agent = FIGHTER_KIND_BUDDY )]
fn frame_buddy(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let energy = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_DAMAGE) as *mut smash::app::KineticEnergy;
        let anti_wind = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_ENV_WIND) as *mut smash::app::KineticEnergy;
        let no_jostle = KineticModule::get_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_JOSTLE) as *mut smash::app::KineticEnergy;
        if [
            *FIGHTER_STATUS_KIND_LANDING,
            *FIGHTER_STATUS_KIND_LANDING_LIGHT,
            *FIGHTER_STATUS_KIND_ATTACK_AIR,
            *FIGHTER_STATUS_KIND_ESCAPE_AIR,
            *FIGHTER_STATUS_KIND_DEAD,
            *FIGHTER_STATUS_KIND_MISS_FOOT,
            *FIGHTER_STATUS_KIND_DAMAGE,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL,
            *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR,
            *FIGHTER_STATUS_KIND_CLIFF_CATCH,
            *FIGHTER_STATUS_KIND_GLIDE_LANDING,
            *FIGHTER_STATUS_KIND_GLIDE_ATTACK,
            *FIGHTER_STATUS_KIND_GLIDE_END
        ].contains(&status_kind)  { 
            macros::STOP_SE(fighter, Hash40::new("se_buddy_glide_loop"));
        };
        if status_kind == *FIGHTER_STATUS_KIND_GLIDE {
            if MotionModule::frame_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING) >= 4.0 && MotionModule::frame_partial(fighter.module_accessor, *FIGHTER_METAKNIGHT_MOTION_PART_SET_KIND_WING) < 5.0 {
                macros::PLAY_SE(fighter, Hash40::new("se_buddy_wing"));
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_S {
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
            smash::app::lua_bind::KineticEnergy::clear_speed(no_jostle);
        };
        if status_kind == *FIGHTER_BUDDY_STATUS_KIND_SPECIAL_S_DASH {
            smash::app::lua_bind::KineticEnergy::clear_speed(energy);
            smash::app::lua_bind::KineticEnergy::clear_speed(anti_wind);
            smash::app::lua_bind::KineticEnergy::clear_speed(no_jostle);
            fighter.sub_air_check_fall_common();
            fighter.sub_wait_ground_check_common(false.into());
            WorkModule::set_int(fighter.module_accessor, 5, *FIGHTER_BUDDY_INSTANCE_WORK_ID_INT_SPECIAL_S_REMAIN);
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -10.0, 0);
            }
            if MotionModule::frame(fighter.module_accessor) > 15.0 {
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                        fighter.change_status(FIGHTER_STATUS_KIND_ESCAPE_AIR.into(), true.into());
                    }
                }
            }
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_buddy
    );
}
