use crate::imports::BuildImports::*; 

const EDGE_VTABLE_FRAME_OFFSET: usize = 0x9db070; //Edge only

#[skyline::hook(offset = EDGE_VTABLE_FRAME_OFFSET)]
unsafe extern "C" fn vtable_edge_Frame(vtable: u64, fighter: &mut Fighter) {
    let module_accessor = fighter.battle_object.module_accessor;
    let status_kind = StatusModule::status_kind(module_accessor);
    let color = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);     
    let BANDANA = color >= 120 && color <= 127;
    if BANDANA {
        return;
    }
    original!()(vtable, fighter)
}

pub fn install() {
	skyline::install_hooks!(
        vtable_edge_Frame
    );
}