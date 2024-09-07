use super::*;
use crate::imports::BuildImports::*;

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_sonic_jumpaerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ROCKETBELT) {
        let energy = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_ROCKETBELT_BURNER_ENERGY_VALUE);
        if 0.0 < energy {
            ItemModule::set_attach_item_action(fighter.module_accessor, ItemKind(*ITEM_KIND_ROCKETBELT), *ITEM_ROCKETBELT_ACTION_JUMP_JET_FIRE, 1.0);
        }
    }
    fighter.status_JumpAerialSub(false.into(), false.into());
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_JumpAerial_Main as *const () as _))
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP_AERIAL, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_sonic_screwjumpaerial_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_ItemScrewJumpAerialSub();
    fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_status_ItemScrewJumpAerial_Main as *const () as _))
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn status_throwk_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_pre_ThrowKirby()
}
#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_INIT_STATUS)]
unsafe fn status_throwk_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_uniq_process_ThrowKirby_initStatus();
    let hit_stop = 8;
    WorkModule::set_int(fighter.module_accessor, hit_stop, *FIGHTER_STATUS_THROW_WORK_INT_STOP_FRAME);
    0.into()
}
#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn status_throwk_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_ThrowKirby()
}
#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_EXIT_STATUS)]
unsafe fn status_throwk_exit(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_uniq_process_ThrowKirby_exitStatus()
}
#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
unsafe fn status_throwk_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_end_ThrowKirby()
}

#[status_script(agent = "sonic", status = FIGHTER_STATUS_KIND_THROW_KIRBY, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn status_throwk_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame_drift = 15.0; //Frame to start player-controlled movement. Should be a few frames after START_AIR flag is set in throw.rs
    let frame_fall = game::THROWHI_FRAME_FALL;
    let frame_fallloop = 35.0;
    let frame_land = game::THROWHI_FRAME_LAND + 1.0; //+1 due to set_frame offset

    let current_frame = MotionModule::frame(fighter.module_accessor);

    if current_frame >= frame_land {
        //Temporary fix for popping into the air
        let grounded = GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
        let status = if grounded { FIGHTER_STATUS_KIND_WAIT } else { FIGHTER_STATUS_KIND_FALL };
        let last_frame = if grounded { MotionModule::end_frame(fighter.module_accessor) - 2.0 } else { MotionModule::end_frame(fighter.module_accessor) - 9.0 };
        
        if current_frame >= last_frame { 
            if !grounded {
                let speed = Vector3f { x: 0.0, y: -0.1, z: 0.0 };
                KineticModule::add_speed(fighter.module_accessor, &speed);
            }
            fighter.change_status(status.into(), false.into());
        }
        return false.into();
    }
    //If we go past a certain frame, then freeze animation and accel downwards
    if current_frame >= frame_fallloop {
        if current_frame < (2.0 + frame_fallloop) { println!("PAUSE"); }
        MotionModule::set_rate(fighter.module_accessor, 0.0);
        let speed = Vector3f { x: 0.0, y: -0.375, z: 0.0 };
        KineticModule::add_speed(fighter.module_accessor, &speed);
    }
    //Start falling
    else if current_frame > frame_fall {
        if current_frame < 2.0 + frame_fall { println!("FALL"); }
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
    }
    
    //Control movement in air
    if current_frame >= frame_drift {
        let speed_max = 0.75;
        let accel = 0.02;
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_max, 0.0);
        sv_kinetic_energy!(set_limit_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, speed_max, 0.0);
        sv_kinetic_energy!(controller_set_accel_x_add, fighter, accel);
    }

    //Groundcast to see if we touched the ground (only after falling), then cut to the landing frame
    if current_frame >= frame_fall
    && GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        MotionModule::set_frame_sync_anim_cmd(fighter.module_accessor, frame_land, true, true, false);
        println!("Knuckles Landed!");
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        //This will make knuckles stick to the ground after landing, might look funny on moving plats
        //GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        status_sonic_jumpaerial_main,
        status_sonic_screwjumpaerial_main,
        status_throwk_pre,
        status_throwk_init,
        status_throwk_main,
        status_throwk_exit,
        status_throwk_end,
        status_throwk_exec
    );
}