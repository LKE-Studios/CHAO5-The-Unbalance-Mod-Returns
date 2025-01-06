use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_demon_Main(fighter : &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let lr = PostureModule::lr(fighter.module_accessor);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_SPECIAL_N);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_DISABLE_AIR_SPECIAL_S);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_DEMON_INSTANCE_WORK_ID_FLAG_ENABLE_RAGE_SYSTEM) {
        DamageModule::set_damage_mul_2nd(fighter.module_accessor, 0.25);
        DamageModule::set_reaction_mul(fighter.module_accessor, 0.25);
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
        AttackModule::set_power_up(fighter.module_accessor, 1.0);
        AttackModule::set_reaction_mul(fighter.module_accessor, 1.0);
    };
    if [*FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2S, *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2F, 
    *FIGHTER_DEMON_STATUS_KIND_ATTACK_STEP_2L].contains(&status_kind) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
}

pub fn install() {
    Agent::new("demon")
    .on_line(Main, frame_demon_Main)
    .install();
}