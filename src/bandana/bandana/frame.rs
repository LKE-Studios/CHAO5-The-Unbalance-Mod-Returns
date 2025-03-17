use crate::imports::BuildImports::*;

pub unsafe extern "C" fn frame_bandana_Main(fighter : &mut L2CFighterCommon) {
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let BANDANA = color >= 120 && color <= 127;
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    if BANDANA {
        let scale = WorkModule::get_param_float(fighter.module_accessor, hash40("scale"), 0);
        if ModelModule::scale(fighter.module_accessor) == scale {
            ModelModule::set_scale(fighter.module_accessor, 0.9);
            AttackModule::set_attack_scale(fighter.module_accessor, 0.9, true);
            GrabModule::set_size_mul(fighter.module_accessor, 0.9);
        };
        if status_kind == *FIGHTER_STATUS_KIND_ENTRY || !sv_information::is_ready_go() {
            let custom_hurtboxes = [
                [hash40("toer") as f64, 0.0, 0.0, 0.0, 1.6, 0.0, 0.0, 1.8, *COLLISION_PART_BODY_LEGS as f64, *HIT_HEIGHT_LOW as f64],
                [hash40("toel") as f64, 0.0, 0.0, 0.0, 1.6, 0.0, 0.0, 1.8, *COLLISION_PART_BODY_LEGS as f64, *HIT_HEIGHT_LOW as f64],
                [hash40("body") as f64, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 5.2, *COLLISION_PART_BODY as f64, *HIT_HEIGHT_HIGH as f64]
            ];
            let mut f = 0;
            for i in custom_hurtboxes {
                let mut vec1 = Vector3f{x: i[1] as f32, y: i[2] as f32, z: i[3] as f32};
                let mut vec2 = Vector3f{x: i[4] as f32, y: i[5] as f32, z: i[6] as f32};
                FighterUtil::set_hit_data(fighter.module_accessor, f, 0, &vec1, &vec2,i[7] as f32, Hash40::new_raw(i[0] as u64), CollisionPart(i[8] as i32), HitHeight(i[9] as i32), HitStatus(*HIT_STATUS_NORMAL), CollisionShapeType(*COLLISION_SHAPE_TYPE_CAPSULE));    
                f += 1;
            }
        }
        if motion_kind == hash40("jump_aerial_f1") {
            if frame < 20.0 {
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
                WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
            }
            else {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_ATTACK_HI4 {
            let effect_handle = WorkModule::get_int(fighter.module_accessor, *FIGHTER_BANDANA_INSTANCE_WORK_ID_INT_ATTACK_HI4_EFFECT_HANDLE);
            if frame == 15.0 {
                let attack_hi4_effect = EffectModule::req_follow(fighter.module_accessor, Hash40::new("kirby_vacuum"), Hash40::new("top"), &Vector3f{x: 0.0, y: 9.0, z: 0.0}, &Vector3f{x: -90.0, y: 90.0, z: 0.0}, 0.6, true, 0, 0, 0, 0, 0, true, true);
                WorkModule::set_int(fighter.module_accessor, attack_hi4_effect as i32, *FIGHTER_BANDANA_INSTANCE_WORK_ID_INT_ATTACK_HI4_EFFECT_HANDLE);
                EffectModule::set_scale(fighter.module_accessor, effect_handle as u32, &Vector3f{x: 1.25, y: 0.6, z: 0.6});
            }
            if frame >= 47.0 {
                EffectModule::set_scale(fighter.module_accessor, effect_handle as u32, &Vector3f{x: 0.0, y: 0.0, z: 0.0});
                EffectModule::kill_kind(fighter.module_accessor, Hash40::new("kirby_vacuum"), false, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if frame == 100.0 {
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_warpstar_break"), Hash40::new("top"), &Vector3f{x: 0.0, y: 1.5, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0.33, true, 0, 0, 0, 0, 0, true, true);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_warpstar_break"), Hash40::new("top"), &Vector3f{x: 0.0, y: 1.5, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0.33, true, 0, 0, 0, 0, 0, true, true);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_warpstar_break"), Hash40::new("top"), &Vector3f{x: 0.0, y: 1.5, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0.33, true, 0, 0, 0, 0, 0, true, true);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_warpstar_break"), Hash40::new("top"), &Vector3f{x: 0.0, y: 1.5, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0.33, true, 0, 0, 0, 0, 0, true, true);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_warpstar_break"), Hash40::new("top"), &Vector3f{x: 0.0, y: 1.5, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0.33, true, 0, 0, 0, 0, 0, true, true);
                EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_warpstar_break"), Hash40::new("top"), &Vector3f{x: 0.0, y: 1.5, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 0.33, true, 0, 0, 0, 0, 0, true, true);
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_LW {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                DamageModule::heal(fighter.module_accessor, -16.0, 0);
                if DamageModule::damage(fighter.module_accessor, 0) > 0.0 {
                    EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_recovery"), Hash40::new("top"), &VECTOR_ZERO, &VECTOR_ZERO, 1.0, true, 0, 0, 0, 0, 0, true, true);
                    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_lifeup"), true, false, false, false, enSEType(0));
                }
            }
        };
    }
}

pub unsafe extern "C" fn frame_bandana_Exec(fighter: &mut L2CFighterCommon) {}

pub unsafe extern "C" fn frame_bandana_spear2_Exec(weapon: &mut L2CFighterBase) {
    let status_kind = StatusModule::status_kind(weapon.module_accessor);
    let motion_kind = MotionModule::motion_kind(weapon.module_accessor);
    let frame = MotionModule::frame(weapon.module_accessor);
    if motion_kind == hash40("stick") {
        if frame == 1.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: 4.71623, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 2.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: -4.71623, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 3.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: -4.71623, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 4.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: 4.71623, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 5.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: 4.48, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 6.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: -4.48, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 7.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: -4.48, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 8.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: 4.48, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 9.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: 3.5, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 10.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: -3.5, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 11.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: -3.5, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 12.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: 3.5, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 13.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: 2.15, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 14.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: -2.15, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 15.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: -2.15, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
        else if frame == 16.0 {
            ModelModule::set_joint_rotate(weapon.module_accessor, Hash40::new_raw(hash40("root")), &Vector3f{x: 2.15, y: 0.0, z: 0.0}, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8}, MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});
        }
    }
}

pub fn install() {
    Agent::new("edge")
    .on_line(Main, frame_bandana_Main)
    .on_line(Exec, frame_bandana_Exec)
    .install();
    
    Agent::new("edge_spear2")
    .on_line(Exec, frame_bandana_spear2_Exec)
    .install();
}