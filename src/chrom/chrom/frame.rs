use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_chrom_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let stick_x = ControlModule::get_stick_x(fighter.module_accessor);
    if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_HI || status_kind == *FIGHTER_ROY_STATUS_KIND_SPECIAL_HI_2 {
        if situation_kind == *SITUATION_KIND_AIR {
            if stick_x != 0.0 {
                KineticModule::add_speed_outside(fighter.module_accessor, *KINETIC_OUTSIDE_ENERGY_TYPE_WIND_NO_ADDITION, &Vector3f{x: 0.2 * stick_x.signum(), y: 0.0, z: 0.0});
            }
        }
    };
}

pub fn install() {
    Agent::new("chrom")
    .on_line(Main, frame_chrom_Main)
    .install();
}