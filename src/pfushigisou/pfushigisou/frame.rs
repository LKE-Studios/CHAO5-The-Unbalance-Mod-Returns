use crate::imports::BuildImports::*;

pub static mut GFX_COUNTER : [i32; 8] = [0; 8];
pub static mut EFFECT_SCALE_MUL : [f32; 8] = [0.01; 8];
static EFFECT_SCALE_MIN : f32 = 0.01;
static EFFECT_SCALE_MAX : f32 = 0.35;

unsafe extern "C" fn frame_pfushigisou_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) 
    && ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_GUARD, false);
    }
    let mut charge = WorkModule::get_int(fighter.module_accessor, FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_GUARD_WORK_INT_CHARGE);
    let mut effect_counter = WorkModule::get_int(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_INT_EFFECT_COUNTER);
    let mut effect_scale_mul = WorkModule::get_float(fighter.module_accessor, FIGHTER_INSTANCE_WORK_ID_FLOAT_EFFECT_SCALE_MUL);
    let charge_effect_scale_min = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_guard"), hash40("charge_effect_scale_min"));
    let charge_effect_scale_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_guard"), hash40("charge_effect_scale_max"));
    if charge >= 1 {
        effect_counter += 1;
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) 
        && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
            effect_scale_mul += 0.001;
        }
        else {
            effect_scale_mul += 0.0;
        }
        effect_scale_mul = effect_scale_mul.clamp(charge_effect_scale_min, charge_effect_scale_max);
        if effect_counter >= 8 {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_hit_fire"), Hash40::new("flowerc"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, effect_scale_mul, true, 0, 0, 0, 0, 0, true, true);
            LAST_EFFECT_SET_COLOR(fighter, /*R*/ 1.5, /*G*/ 1.0, /*B*/ 0.0);
            effect_counter = 0;
        }
    } 
    else {
        effect_scale_mul = charge_effect_scale_min;
    }
    if !sv_information::is_ready_go() {
        WorkModule::set_int(fighter.module_accessor, 0, FIGHTER_PFUSHIGISOU_STATUS_SPECIAL_GUARD_WORK_INT_CHARGE);
        WorkModule::set_float(fighter.module_accessor, charge_effect_scale_min, FIGHTER_INSTANCE_WORK_ID_FLOAT_EFFECT_SCALE_MUL);
    };
}

pub fn install() {
    Agent::new("pfushigisou")
    .on_line(Main, frame_pfushigisou_Main)
    .install();
}