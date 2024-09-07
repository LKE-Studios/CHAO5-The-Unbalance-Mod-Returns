use crate::imports::BuildImports::*;

#[status_script(agent = "pacman", status = FIGHTER_STATUS_KIND_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_pacman_jumpaerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_JumpAerial()
}

pub fn install() {
    install_status_scripts!(
        status_pacman_jumpaerial_main
    );
}