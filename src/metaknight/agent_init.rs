use {
    smash::{
        lua2cpp::L2CFighterCommon,
        app::{lua_bind::*, *},
        lib::{lua_const::*, L2CValue}
    },
    smashline::*,
    super::frame::USED_SPECIAL_N
};

pub unsafe extern "C" fn metaknight_used_special_n(fighter: &mut L2CFighterCommon) -> L2CValue {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    (!USED_SPECIAL_N[entry_id]).into()
}

#[fighter_reset]
fn agent_reset(fighter: &mut L2CFighterCommon) {
    unsafe {
        let fighter_kind = utility::get_kind(&mut *fighter.module_accessor);
        if fighter_kind != *FIGHTER_KIND_METAKNIGHT {
            return;
        }
        fighter.global_table[0x38].assign(&L2CValue::Ptr(metaknight_used_special_n as *const () as _));
        // 0x38 is for SPECIAL N
        // 0x39 is for SPECIAL S
        // 0x3A is for SPECIAL HI
        // 0x3B is for SPECIAL LW
    }
}

pub fn install() {
    install_agent_resets!(
        agent_reset
    );
}
