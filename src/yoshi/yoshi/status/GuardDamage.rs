use crate::imports::BuildImports::*;
use crate::yoshi::yoshi::status::Guard::yoshi_guard_exec_helper;

unsafe extern "C" fn status_yoshi_GuardDamage_Exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_ftStatusUniqProcessGuardDamage_execStatus_common();
    yoshi_guard_exec_helper(fighter);
    0.into()
}

unsafe extern "C" fn status_yoshi_GuardDamage_ExecStop(fighter: &mut L2CFighterCommon) -> L2CValue {
    yoshi_guard_exec_helper(fighter);
    fighter.sub_ftStatusUniqProcessGuardOn_execStop_Inner(L2CValue::Void());
    0.into()
}

pub fn install() {
    Agent::new("yoshi")
    .status(Exec, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, status_yoshi_GuardDamage_Exec)
    .status(ExecStop, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, status_yoshi_GuardDamage_ExecStop)
    .install();
}