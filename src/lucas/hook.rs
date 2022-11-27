use smash::lib::lua_const::*;
use smash::app::*;
use smash::hash40;
use smash::app::lua_bind::*;
//use crate::prelude::*;

static mut FLOAT_OFFSET : usize = 0x4e53c0;

#[skyline::hook(offset=FLOAT_OFFSET)] //Claus exclusive fighter attributes
pub unsafe fn claus_fighterparam(
boma: u64,
param_type: u64,
param_hash: u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    if fighter_kind == FIGHTER_KIND_LUCAS
    && color >= 8 && color <= 15
    && param_hash == 0 {
        if param_type == hash40("walk_speed_max") {
            return 1.47; //Lucas 1.65
        }
        if param_type == hash40("dash_speed") {
            return 2.24; //Lucas 2.15
        }
        else if param_type == hash40("run_speed_max") {
            return 1.9; //Lucas 1.824
        }
        else if param_type == hash40("air_speed_x_stable") {
            return 2.048; //Lucas 1.885
        }
        else if param_type == hash40("dive_speed_y") {
            return 2.35; //Lucas 2.242
        }
    }
ret
}

pub fn install() {
    skyline::install_hooks!(claus_fighterparam);
}