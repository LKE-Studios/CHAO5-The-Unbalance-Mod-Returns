use smash::{
    *,
    phx::Hash40,
    lib::lua_const::*,
    app::{lua_bind::*, *}
};

extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app14FighterManagerEE9instance_E"]
    pub static FIGHTER_MANAGER: *mut smash::app::FighterManager;
}
extern "C"{
    #[link_name = "_ZN3lib9SingletonIN3app19FighterCutInManagerEE9instance_E"]
    pub static FIGHTER_CUTIN_MANAGER: *mut smash::app::FighterCutInManager;
}

pub fn get_module_accessor_by_entry_id(entry_id: i32) -> *mut smash::app::BattleObjectModuleAccessor {
    unsafe {
        &mut *smash::app::sv_battle_object::module_accessor(smash::app::Fighter::get_id_from_entry_id(entry_id))
    }
}

pub fn is_grounded(module_accessor: *mut app::BattleObjectModuleAccessor) -> bool {
    let situation_kind = unsafe { StatusModule::situation_kind(module_accessor) as i32 };
    situation_kind == SITUATION_KIND_GROUND
}

pub unsafe fn set_position_lock(entry_id: i32){
    lua_bind::FighterManager::set_position_lock(FIGHTER_MANAGER, FighterEntryID(entry_id), true);
}

pub unsafe fn unset_position_lock(entry_id: i32){
    lua_bind::FighterManager::set_position_lock(FIGHTER_MANAGER, FighterEntryID(entry_id), false);
}


pub unsafe fn change_motion(module_accessor: *mut BattleObjectModuleAccessor, anim: &str){
    MotionModule::change_motion(module_accessor, Hash40::new(anim), 0.0, 1.0, false, 0.0, false, false);
}
pub unsafe fn get_entry_id(module_accessor: *mut BattleObjectModuleAccessor) -> usize{
    WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize
}

pub unsafe fn disable_gravity(module_accessor: *mut BattleObjectModuleAccessor){
    KineticModule::unable_energy(module_accessor,  *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
}

pub unsafe fn enable_gravity(module_accessor: *mut BattleObjectModuleAccessor){
    KineticModule::enable_energy(module_accessor,  *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
}
/*
todo
pub unsafe fn get_nearest_opponent(module_accessor: *mut BattleObjectModuleAccessor) -> i32{

    let entry_id = get_entry_id(module_accessor);
    let mut lowestavg = 0.0;
    let entry_count = FighterManager::entry_count(FIGHTER_MANAGER);
    for i in 0..entry_count{
        let curr_boma = get_module_accessor_by_entry_id(i);
        let avg = (PostureModule::pos_x(curr_boma) + PostureModule::pos_y(curr_boma)) / 2;
    }
}
 */
