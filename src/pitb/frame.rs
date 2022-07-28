use smash::lib::lua_const::*;
use smash::app::lua_bind::*;
use smashline::*;
use smash::lua2cpp::L2CFighterCommon;

/*static mut FLOAT : [i32; 8] = [0; 8]; //Logs Float Time
static mut START_FLOAT : [bool; 8] = [false; 8];
static mut CHECK_FLOAT : [i32; 8] = [0; 8];
static mut CHECK_FLOAT_MAX : i32 = 10; //Frames where jump needs to be held to start floating
static mut X : [f32; 8] = [0.0; 8]; //Logs speed
static mut Y : [f32; 8] = [0.0; 8]; //Logs speed
static mut FLOAT_MAX : i32 = 1800; //Frames this bitch can float (In frames, 300 = 5 seconds)
static mut X_MAX : f32 = 2.123; //Max Horizontal movespeed
// static mut X_ACCEL_ADD : f32 = 0.06; //Air Accel Add
static mut X_ACCEL_MUL : f32 = 0.09; //Air Accel Mul
static mut Y_MAX : f32 = 1.224; //Max Vertical movespeed
// static mut Y_ACCEL_ADD : f32 = 0.06;
// static mut Y_ACCEL_MUL : f32 = 0.06;*/
static mut HOLD_TIME : [f32; 8] = [0.0; 8];

#[fighter_frame( agent = FIGHTER_KIND_PITB )]
fn pitb_opff(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if ![*FIGHTER_STATUS_KIND_GLIDE, *FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR{
            if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP){
                HOLD_TIME[ENTRY_ID] +=1.0;
            }
            if HOLD_TIME[ENTRY_ID] == 20.0{
                fighter.change_status(FIGHTER_STATUS_KIND_GLIDE.into(), true.into());
            }
        }
        else{
            HOLD_TIME[ENTRY_ID] = 0.0;
        };
        if [*FIGHTER_STATUS_KIND_FALL_SPECIAL].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_JUMP].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_WAIT].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
        if [*FIGHTER_STATUS_KIND_REBIRTH].contains(&status_kind) && HOLD_TIME[ENTRY_ID] > 1.0{
            HOLD_TIME[ENTRY_ID] = 1.0;
        };
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        pitb_opff
    );
}
