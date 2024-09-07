use crate::imports::BuildImports::*;
use crate::pfushigisou::status::*;
use crate::pfushigisou::status::SFX_COUNTER;

pub static mut GFX_COUNTER : [i32; 8] = [0; 8];
pub static mut EFFECT_SCALE_MUL : [f32; 8] = [0.01; 8];
static EFFECT_SCALE_MIN : f32 = 0.01;
static EFFECT_SCALE_MAX : f32 = 0.35;

#[fighter_frame( agent = FIGHTER_KIND_PFUSHIGISOU )]
fn frame_pfushigisou(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_ESCAPE_AIR].contains(&status_kind) && 
        ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            StatusModule::change_status_request_from_script(fighter.module_accessor, FIGHTER_PFUSHIGISOU_STATUS_KIND_SPECIAL_N2_CHARGE, false);
        }
        if PFUSHIGISOU_SOLAR_BEAM_TIMER[ENTRY_ID] >= 1 {
            GFX_COUNTER[ENTRY_ID] += 1;
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && 
            ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                EFFECT_SCALE_MUL[ENTRY_ID] += 0.001;
            }
            else {
                EFFECT_SCALE_MUL[ENTRY_ID] += 0.0;
            }
            EFFECT_SCALE_MUL[ENTRY_ID] = EFFECT_SCALE_MUL[ENTRY_ID].clamp(EFFECT_SCALE_MIN, EFFECT_SCALE_MAX);
            if GFX_COUNTER[ENTRY_ID] >= 8 {
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("pfushigisou_atk_hi4"), Hash40::new("flowerc"), &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, &Vector3f { x: 0.0, y: 0.0, z: 0.0 }, EFFECT_SCALE_MUL[ENTRY_ID], true, 0, 0, 0, 0, 0, true, true);
                GFX_COUNTER[ENTRY_ID] = 0;
            }
        } 
        else {
            EFFECT_SCALE_MUL[ENTRY_ID] = EFFECT_SCALE_MIN;
            SFX_COUNTER[ENTRY_ID] = 0;
        }
        if sv_information::is_ready_go() == false {
            PFUSHIGISOU_SOLAR_BEAM_TIMER[ENTRY_ID] = 0;
            EFFECT_SCALE_MUL[ENTRY_ID] = 0.01;
            SFX_COUNTER[ENTRY_ID] = 0;
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_pfushigisou
    );
}