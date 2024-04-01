use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_rosetta_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    if status_kind == *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_HI_JUMP
	&& !StatusModule::is_changing(fighter.module_accessor) && frame > 2.0 {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ESCAPE_AIR, false);
        }
    }
    if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_N_SHOOT,
        *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_N_CHARGE, *FIGHTER_ROSETTA_STATUS_KIND_SPECIAL_N_RETURN].contains(&status_kind) {
        if !fighter.is_in_hitlag() && !StatusModule::is_changing(fighter.module_accessor) && situation_kind == *SITUATION_KIND_AIR {
            fighter.sub_air_check_dive();
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
                if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR || 
                KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE {
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                    let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                    fighter.clear_lua_stack();
                    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                    sv_kinetic_energy::enable(fighter.lua_state_agent);
                    KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                }
            }
        }
    }
}

unsafe extern "C" fn frame_rosetta_tico_Main(weapon: &mut L2CFighterBase) {
    let module_accessor = sv_system::battle_object_module_accessor(weapon.lua_state_agent);
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let status_kind = StatusModule::status_kind(weapon.module_accessor);
    let weapon_kind = utility::get_kind(module_accessor) as i32;
    let motion_kind = MotionModule::motion_kind(module_accessor);
    let owner_motion_kind = MotionModule::motion_kind(owner_module_accessor);
    let owner_status_kind = StatusModule::status_kind(owner_module_accessor);
    if weapon_kind == WEAPON_KIND_ROSETTA_TICO {
        if [*FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE].contains(&owner_status_kind)
        || [*WEAPON_ROSETTA_TICO_STATUS_KIND_FREE_GUARD, *WEAPON_ROSETTA_TICO_STATUS_KIND_FOLLOW_GUARD].contains(&status_kind)
        || motion_kind == hash40("follow_guard") {
            HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        }
        if owner_status_kind == *FIGHTER_STATUS_KIND_GUARD_OFF && owner_motion_kind != hash40("just_shield_off") || owner_status_kind == *FIGHTER_STATUS_KIND_FURAFURA {
            HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);	
        }
        if owner_status_kind == *WEAPON_ROSETTA_TICO_STATUS_KIND_SPECIAL_N_SHOOT {
            HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
        }
        if owner_status_kind == *WEAPON_ROSETTA_TICO_STATUS_KIND_SPECIAL_N_END {
            HitModule::set_whole(module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        }
    }
}

pub fn install() {
    Agent::new("rosetta")
    .on_line(Main, frame_rosetta_Main)
    .install();
    
    Agent::new("rosetta_tico")
    .on_line(Main, frame_rosetta_tico_Main)
    .install();
}