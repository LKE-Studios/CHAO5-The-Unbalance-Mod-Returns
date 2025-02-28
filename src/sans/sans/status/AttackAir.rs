use crate::imports::BuildImports::*;

unsafe extern "C" fn status_sans_AttackAir_CheckAttack(fighter: &mut L2CFighterCommon, _param_2: &L2CValue, param_3: &L2CValue) -> L2CValue {
    let module_accessor = sv_system::battle_object_module_accessor(fighter.lua_state_agent); 
    let table = param_3.get_table() as *mut smash2::lib::L2CTable;
    let category = get_table_value(table, "object_category_").try_integer().unwrap() as i32;
    let collision_kind = get_table_value(table, "kind_").try_integer().unwrap() as i32;
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SANS = color >= 120 && color <= 127;
    if category == *BATTLE_OBJECT_CATEGORY_FIGHTER {
        if SANS && utility::get_kind(module_accessor) == *FIGHTER_KIND_PALUTENA {
            if collision_kind == *COLLISION_KIND_HIT {
                if [hash40("attack_air_f")].contains(&MotionModule::motion_kind(module_accessor)) {
                    SoundModule::play_se(module_accessor, Hash40::new("se_palutena_attackair_b01"), true, false, false, false, enSEType(0));
                }
            }
        }
    }
    0.into()
}

pub fn install() {
    Agent::new("palutena")
    .status(CheckAttack, *FIGHTER_STATUS_KIND_ATTACK_AIR, status_sans_AttackAir_CheckAttack)
    .install();
}