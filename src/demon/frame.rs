use crate::imports::BuildImports::*;

static mut FLOAT : [i32; 8] = [0; 8]; //Logs Float Time
static mut START_FLOAT : [bool; 8] = [false; 8];
static mut CHECK_FLOAT : [i32; 8] = [0; 8];
static mut CHECK_FLOAT_MAX : i32 = 16; //Frames where jump needs to be held to start floating
static mut X : [f32; 8] = [0.0; 8]; //Logs speed
static mut Y : [f32; 8] = [0.0; 8]; //Logs speed
static mut FLOAT_MAX : i32 = 900; //Frames this bitch can float (In frames, 300 = 5 seconds)
static mut X_MAX : f32 = 2.2; //Max Horizontal movespeed
static mut X_ACCEL_MUL : f32 = 0.09; //Air Accel Mul
static mut Y_MAX : f32 = 2.15; //Max Vertical movespeed

#[fighter_frame( agent = FIGHTER_KIND_DEMON )]
pub fn frame_demon(fighter : &mut L2CFighterCommon) {
    unsafe {
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM) {
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 0.4);
            DamageModule::set_reaction_mul(fighter.module_accessor, 0.4);
        }
        else {
            DamageModule::set_damage_mul_2nd(fighter.module_accessor, 1.0);
            DamageModule::set_reaction_mul(fighter.module_accessor, 1.0);
        };
        if [*FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE, *FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE_CATCH, 
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE_FALL, *FIGHTER_DEMON_STATUS_KIND_ATTACK_RAGE_GROUND].contains(&status_kind) {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
            AttackModule::set_power_up(fighter.module_accessor, 3.0);
            AttackModule::set_reaction_mul(fighter.module_accessor, 1.25);
        }
        else {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
            AttackModule::set_power_up(fighter.module_accessor, 1.0);
            AttackModule::set_reaction_mul(fighter.module_accessor, 1.0);
        };
        if [*FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F, 
        *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L].contains(&status_kind) {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
        }
        else {
            damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
        }
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        frame_demon
    );
}