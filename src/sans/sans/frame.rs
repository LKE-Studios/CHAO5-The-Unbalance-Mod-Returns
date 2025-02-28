use crate::imports::BuildImports::*;

unsafe extern "C" fn frame_sans_Main(fighter: &mut L2CFighterCommon) {
    let status_kind = StatusModule::status_kind(fighter.module_accessor);
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let ENTRY_ID = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let frame = MotionModule::frame(fighter.module_accessor);
    let situation_kind = StatusModule::situation_kind(fighter.module_accessor);
    let color = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR);
    let SANS = color >= 120 && color <= 127; 
    if SANS {
        if situation_kind == *SITUATION_KIND_AIR {
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_ENABLE);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT);
            WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_JUMP_FLAG_GLIDE_INPUT_BACK);
        }
        let scale = WorkModule::get_param_float(fighter.module_accessor, hash40("scale"), 0);
        if ModelModule::scale(fighter.module_accessor) == scale {
            ModelModule::set_scale(fighter.module_accessor, 1.15);
            AttackModule::set_attack_scale(fighter.module_accessor, 1.15, true);
            GrabModule::set_size_mul(fighter.module_accessor, 1.15);
        }
        if status_kind == *FIGHTER_STATUS_KIND_ENTRY || !sv_information::is_ready_go() {
            WorkModule::set_int(fighter.module_accessor, 1, *FIGHTER_SANS_INSTANCE_WORK_ID_INT_BONE_TYPE);
            let custom_hurtboxes = [
                //["bone", x1, y1, z1, x2, y2, z2, scale, collision_part, hit height]
                [hash40("head") as f64, 2.0, 0.0, 0.0, 0.0, 0.0, 0.0, 2.5, *COLLISION_PART_BODY_LEGS as f64, *HIT_HEIGHT_LOW as f64]
            ];
            let mut f = 0;
            for i in custom_hurtboxes {
                let mut vec1 = Vector3f{x: i[1] as f32, y: i[2] as f32, z: i[3] as f32};
                let mut vec2 = Vector3f{x: i[4] as f32, y: i[5] as f32, z: i[6] as f32};
                FighterUtil::set_hit_data(fighter.module_accessor, f, 0, &vec1, &vec2, i[7] as f32, Hash40::new_raw(i[0] as u64), CollisionPart(i[8] as i32), HitHeight(i[9] as i32), HitStatus(*HIT_STATUS_NORMAL), CollisionShapeType(*COLLISION_SHAPE_TYPE_CAPSULE));    
                f += 1;
            };
        }
        if status_kind == *FIGHTER_STATUS_KIND_SPECIAL_N {
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND
            && StatusModule::prev_situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_LANDING_LIGHT, true);
            }
            if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_FALL);
                if ControlModule::get_command_flag_cat(fighter.module_accessor, 1) & *FIGHTER_PAD_CMD_CAT2_FLAG_FALL_JUMP != 0
                    && ControlModule::get_stick_y(fighter.module_accessor) < -0.66
                    && KineticModule::get_sum_speed_y(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY) <= 0.0 {
                    WorkModule::set_flag(fighter.module_accessor, true, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                }
            }
        }
        if motion_kind == hash40("attack_air_n") {
            if frame >= 39.0 {
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("prop_whoopiecoushin"), false);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("prop_cushionstrap"), false);
            }
            else {
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("prop_whoopiecoushin"), true);
                ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("prop_cushionstrap"), true);
            };
        }
        else {
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("prop_whoopiecoushin"), false);
            ModelModule::set_mesh_visibility(fighter.module_accessor, Hash40::new("prop_cushionstrap"), false);
        }
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_SANS_GENERATE_ARTICLE_GASTER) {
            EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_black"), false, false);
        }
        if status_kind == *FIGHTER_STATUS_KIND_CATCH_ATTACK {
            if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
                if DamageModule::damage(fighter.module_accessor, 0) > 0.0 {
                    SoundModule::play_se(fighter.module_accessor, Hash40::new("se_common_lifeup"), true, false, false, false, enSEType(0));
                    EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_recovery"), Hash40::new("top"), &VECTOR_ZERO, &VECTOR_ZERO, 1.0, true, 0, 0, 0, 0, 0, true, true);
                }
            }
        }
        if [*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_PALUTENA_STATUS_KIND_SPECIAL_HI_3].contains(&status_kind) {
            if !fighter.is_in_hitlag() && !StatusModule::is_changing(fighter.module_accessor) && situation_kind == *SITUATION_KIND_AIR {
                fighter.sub_air_check_dive();
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
                    if KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR || 
                    KineticModule::get_kinetic_type(fighter.module_accessor) == *FIGHTER_KINETIC_TYPE_MOTION_AIR_ANGLE {
                        fighter.clear_lua_stack();
                        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_MOTION);
                        let speed_y = sv_kinetic_energy::get_speed_y(fighter.lua_state_agent);
                        fighter.clear_lua_stack();
                        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY, ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, speed_y, 0.0, 0.0, 0.0);
                        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
                        fighter.clear_lua_stack();
                        lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
                        sv_kinetic_energy::enable(fighter.lua_state_agent);
                        KineticUtility::clear_unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
                    }
                }
            }
        }
    }
}

unsafe extern "C" fn frame_sans_gaster_Exec(weapon: &mut L2CFighterCommon) {
    let owner_module_accessor = sv_system::battle_object_module_accessor(weapon.lua_state_agent); 
    let module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    if WorkModule::is_flag(module_accessor, *FIGHTER_SANS_INSTANCE_WORK_ID_FLAG_GASTER_ANGLE) {
        let mut rotation = Vector3f{x: 20.0, y: 0.0 , z: 0.0 };
        ModelModule::set_joint_rotate(owner_module_accessor, Hash40::new("rot"), &rotation, MotionNodeRotateCompose{_address: *MOTION_NODE_ROTATE_COMPOSE_AFTER as u8},  MotionNodeRotateOrder{_address: *MOTION_NODE_ROTATE_ORDER_XYZ as u8});	
    };
}

pub fn install() {
    Agent::new("palutena")
    .on_line(Main, frame_sans_Main)
    .install();

    Agent::new("palutena_gaster")
    .on_line(Exec, frame_sans_gaster_Exec)
    .install();
}