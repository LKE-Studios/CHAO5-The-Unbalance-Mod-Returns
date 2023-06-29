use crate::imports::BuildImports::*;

pub struct GlideParams {
    pub angle_max_up : f32, //#0 Max Upward Angle
    pub angle_max_down : f32, //#1 Max Downward Angle
    pub v_glide_start : f32, //#2 V Speed added on GlideStart
    pub gravity_start : f32, //#3 Gravity multiplier on GlideStart
    pub speed_mul_start : f32, //#4 H speed multiplier on GlideStart
    pub base_speed : f32, //#5 Base Power/Speed
    pub speed_change : f32, //#6 Power Rate
    pub max_speed : f32, //#7 Maximum Speed
    pub end_speed : f32, //#8 End Speed
    pub gravity_accel : f32, //#9 Gravity Acceleration
    pub gravity_speed : f32, //#10 Gravity Max Speed
    pub angle_extra : f32, //#11 Angle stuff but unknown what this is for
    pub angle_more_speed : f32, //#12 Angle to gain more speed
    pub down_speed_add : f32, //#13 Max added speed gained aiming downward
    pub unknown : f32, //#14 Unknown
    pub radial_stick : f32, //#15 Radial Stick Sensitivity
    pub up_angle_accel : f32, //#16 Upward angular acceleration
    pub down_angle_accel : f32, //#17 Downward angular acceleration
    pub max_angle_speed : f32, //#18 Maximum angular speed
    pub add_angle_speed : f32, //#19 Added angular speed for when stick is center
}

impl GlideParams {
    pub fn get(fighter: &mut L2CFighterCommon) -> GlideParams {
        let kind = fighter.global_table[0x2].get_i32();
        if kind == *FIGHTER_KIND_METAKNIGHT {
            return GlideParams {
                angle_max_up : 80.0, //#0 Max Upward Angle
                angle_max_down : -70.0, //#1 Max Downward Angle
                v_glide_start : 0.75, //#2 V Speed added on GlideStart
                gravity_start : 1.0, //#3 Gravity multiplier on GlideStart
                speed_mul_start : 1.0, //#4 H speed multiplier on GlideStart
                base_speed : 1.8, //#5 Base Power/Speed
                speed_change : 0.015, //#6 Power Rate
                max_speed : 2.3, //#7 Maximum Speed
                end_speed : 0.7, //#8 End Speed
                gravity_accel : 0.03, //#9 Gravity Acceleration
                gravity_speed : 0.5, //#10 Gravity Max Speed
                angle_extra : 15.0, //#11 Angle stuff but currently unused
                angle_more_speed : -25.0, //#12 Angle to gain more speed
                down_speed_add : 0.03, //#13 Max added speed gained aiming downward
                unknown : 0.15, //#14 Unknown, unused
                radial_stick : 0.25, //#15 Radial Stick Sensitivity
                up_angle_accel : 0.55, //#16 Upward angular acceleration
                down_angle_accel : 0.75, //#17 Downward angular acceleration
                max_angle_speed : 7.0, //#18 Maximum angular speed
                add_angle_speed : 1.0 //#19 Added angular speed for when stick is center
            };
        }
        if kind == *FIGHTER_KIND_PIT {
            return GlideParams {
                angle_max_up : 70.0, //#0 Max Upward Angle
                angle_max_down : -60.0, //#1 Max Downward Angle
                v_glide_start : 0.75, //#2 V Speed added on GlideStart
                gravity_start : 1.0, //#3 Gravity multiplier on GlideStart
                speed_mul_start : 1.0, //#4 H speed multiplier on GlideStart
                base_speed : 1.9, //#5 Base Power/Speed
                speed_change : 0.025, //#6 Power Rate
                max_speed : 2.4, //#7 Maximum Speed
                end_speed : 1.0, //#8 End Speed
                gravity_accel : 0.05, //#9 Gravity Acceleration
                gravity_speed : 0.4, //#10 Gravity Max Speed
                angle_extra : 0.0, //#11 Angle stuff but unused
                angle_more_speed : -25.0, //#12 Angle to gain more speed
                down_speed_add : 0.015, //#13 Max added speed gained aiming downward
                unknown : 0.1, //#14 Unknown, unused
                radial_stick : 0.25, //#15 Radial Stick Sensitivity
                up_angle_accel : 2.0, //#16 Upward angular acceleration
                down_angle_accel : 0.5, //#17 Downward angular acceleration
                max_angle_speed : 6.0, //#18 Maximum angular speed
                add_angle_speed : 0.15 //#19 Added angular speed for when stick is center
            };
        }
        if kind == *FIGHTER_KIND_PITB {
            return GlideParams {
                angle_max_up : 70.0, //#0 Max Upward Angle
                angle_max_down : -65.0, //#1 Max Downward Angle
                v_glide_start : 0.75, //#2 V Speed added on GlideStart
                gravity_start : 1.0, //#3 Gravity multiplier on GlideStart
                speed_mul_start : 1.0, //#4 H speed multiplier on GlideStart
                base_speed : 1.9, //#5 Base Power/Speed
                speed_change : 0.02, //#6 Power Rate
                max_speed : 2.4, //#7 Maximum Speed
                end_speed : 0.95, //#8 End Speed
                gravity_accel : 0.05, //#9 Gravity Acceleration
                gravity_speed : 0.4, //#10 Gravity Max Speed
                angle_extra : 0.0, //#11 Angle stuff but unused
                angle_more_speed : -25.0, //#12 Angle to gain more speed
                down_speed_add : 0.015, //#13 Max added speed gained aiming downward
                unknown : 0.1, //#14 Unknown, unused
                radial_stick : 0.25, //#15 Radial Stick Sensitivity
                up_angle_accel : 0.7, //#16 Upward angular acceleration
                down_angle_accel : 1.0, //#17 Downward angular acceleration
                max_angle_speed : 6.5, //#18 Maximum angular speed
                add_angle_speed : 0.15 //#19 Added angular speed for when stick is center
            };
        }
        if kind == *FIGHTER_KIND_PLIZARDON {
            return GlideParams {
                angle_max_up : 50.0, //#0 Max Upward Angle
                angle_max_down : -50.0, //#1 Max Downward Angle
                v_glide_start : 1.2, //#2 V Speed added on GlideStart
                gravity_start : 0.15, //#3 Gravity multiplier on GlideStart
                speed_mul_start : 1.0, //#4 H speed multiplier on GlideStart
                base_speed : 1.4, //#5 Base Power/Speed
                speed_change : 0.02, //#6 Power Rate
                max_speed : 2.1, //#7 Maximum Speed
                end_speed : 0.5, //#8 End Speed
                gravity_accel : 0.06, //#9 Gravity Acceleration
                gravity_speed : 0.4, //#10 Gravity Max Speed
                angle_extra : 0.0, //#11 Angle stuff but unused
                angle_more_speed : -20.0, //#12 Angle to gain more speed
                down_speed_add : 0.01, //#13 Max added speed gained aiming downward
                unknown : 0.1, //#14 Unknown, unused
                radial_stick : 0.25, //#15 Radial Stick Sensitivity
                up_angle_accel : 0.8, //#16 Upward angular acceleration
                down_angle_accel : 0.4, //#17 Downward angular acceleration
                max_angle_speed : 4.0, //#18 Maximum angular speed
                add_angle_speed : 0.15 //#19 Added angular speed for when stick is center
            };
        }
        if kind == *FIGHTER_KIND_RIDLEY {
            return GlideParams {
                angle_max_up : 60.0, //#0 Max Upward Angle
                angle_max_down : -60.0, //#1 Max Downward Angle
                v_glide_start : 1.0, //#2 V Speed added on GlideStart
                gravity_start : 0.0, //#3 Gravity multiplier on GlideStart
                speed_mul_start : 0.2, //#4 H speed multiplier on GlideStart
                base_speed : 1.6, //#5 Base Power/Speed
                speed_change : 0.02, //#6 Power Rate
                max_speed : 2.1, //#7 Maximum Speed
                end_speed : 0.4, //#8 End Speed
                gravity_accel : 0.06, //#9 Gravity Acceleration
                gravity_speed : 0.5, //#10 Gravity Max Speed
                angle_extra : 0.0, //#11 Angle stuff but unused
                angle_more_speed : -20.0, //#12 Angle to gain more speed
                down_speed_add : 0.01, //#13 Max added speed gained aiming downward
                unknown : 0.1, //#14 Unknown, unused
                radial_stick : 0.25, //#15 Radial Stick Sensitivity
                up_angle_accel : 0.65, //#16 Upward angular acceleration
                down_angle_accel : 0.8, //#17 Downward angular acceleration
                max_angle_speed : 5.0, //#18 Maximum angular speed
                add_angle_speed : 0.15 //#19 Added angular speed for when stick is center
            };
        }
        if kind == *FIGHTER_KIND_BUDDY {
            return GlideParams {
                angle_max_up : 65.0, //#0 Max Upward Angle
                angle_max_down : -55.0, //#1 Max Downward Angle
                v_glide_start : 0.75, //#2 V Speed added on GlideStart
                gravity_start : 1.0, //#3 Gravity multiplier on GlideStart
                speed_mul_start : 1.2, //#4 H speed multiplier on GlideStart
                base_speed : 1.7, //#5 Base Power/Speed
                speed_change : 0.0185, //#6 Power Rate
                max_speed : 2.2, //#7 Maximum Speed
                end_speed : 0.6, //#8 End Speed
                gravity_accel : 0.055, //#9 Gravity Acceleration
                gravity_speed : 0.5, //#10 Gravity Max Speed
                angle_extra : 0.0, //#11 Angle stuff but unused
                angle_more_speed : -20.0, //#12 Angle to gain more speed
                down_speed_add : 0.02, //#13 Max added speed gained aiming downward
                unknown : 0.1, //#14 Unknown, unused
                radial_stick : 0.25, //#15 Radial Stick Sensitivity
                up_angle_accel : 0.8, //#16 Upward angular acceleration
                down_angle_accel : 0.75, //#17 Downward angular acceleration
                max_angle_speed : 4.5, //#18 Maximum angular speed
                add_angle_speed : 0.2 //#19 Added angular speed for when stick is center
            };
        }
        if kind == *FIGHTER_KIND_TRAIL {
            return GlideParams {
                angle_max_up : 40.0, //#0 Max Upward Angle
                angle_max_down : -45.0, //#1 Max Downward Angle
                v_glide_start : 1.5, //#2 V Speed added on GlideStart
                gravity_start : 1.0, //#3 Gravity multiplier on GlideStart
                speed_mul_start : 1.0, //#4 H speed multiplier on GlideStart
                base_speed : 1.5, //#5 Base Power/Speed
                speed_change : 0.025, //#6 Power Rate
                max_speed : 2.0, //#7 Maximum Speed
                end_speed : 0.9, //#8 End Speed
                gravity_accel : 0.04, //#9 Gravity Acceleration
                gravity_speed : 0.4, //#10 Gravity Max Speed
                angle_extra : 0.0, //#11 Angle stuff but unused
                angle_more_speed : -15.0, //#12 Angle to gain more speed
                down_speed_add : 0.01, //#13 Max added speed gained aiming downward
                unknown : 0.1, //#14 Unknown, unused
                radial_stick : 0.25, //#15 Radial Stick Sensitivity
                up_angle_accel : 1.3, //#16 Upward angular acceleration
                down_angle_accel : 0.95, //#17 Downward angular acceleration
                max_angle_speed : 3.0, //#18 Maximum angular speed
                add_angle_speed : 0.15 //#19 Added angular speed for when stick is center
            };
        }
        if kind == *FIGHTER_KIND_PALUTENA {
            return GlideParams {
                angle_max_up : 75.0, //#0 Max Upward Angle
                angle_max_down : -65.0, //#1 Max Downward Angle
                v_glide_start : 0.0, //#2 V Speed added on GlideStart
                gravity_start : 0.0, //#3 Gravity multiplier on GlideStart
                speed_mul_start : 0.0, //#4 H speed multiplier on GlideStart
                base_speed : 1.7, //#5 Base Power/Speed
                speed_change : 0.02, //#6 Power Rate
                max_speed : 2.5, //#7 Maximum Speed
                end_speed : 0.8, //#8 End Speed
                gravity_accel : 0.035, //#9 Gravity Acceleration
                gravity_speed : 0.4, //#10 Gravity Max Speed
                angle_extra : 0.0, //#11 Angle stuff but unused
                angle_more_speed : -25.0, //#12 Angle to gain more speed
                down_speed_add : 0.02, //#13 Max added speed gained aiming downward
                unknown : 0.1, //#14 Unknown, unused
                radial_stick : 0.25, //#15 Radial Stick Sensitivity
                up_angle_accel : 1.6, //#16 Upward angular acceleration
                down_angle_accel : 0.65, //#17 Downward angular acceleration
                max_angle_speed : 6.5, //#18 Maximum angular speed
                add_angle_speed : 0.15 //#19 Added angular speed for when stick is center
            };
        }
        else {
            // if fighter kind not defined, just use Brawl Meta Knight's params.
            return GlideParams {
                angle_max_up : 80.0, //#0 Max Upward Angle
                angle_max_down : -70.0, //#1 Max Downward Angle
                v_glide_start : 0.75, //#2 V Speed added on GlideStart
                gravity_start : 1.0, //#3 Gravity multiplier on GlideStart
                speed_mul_start : 1.0, //#4 H speed multiplier on GlideStart
                base_speed : 1.7, //#5 Base Power/Speed
                speed_change : 0.04, //#6 Power Rate
                max_speed : 2.2, //#7 Maximum Speed
                end_speed : 0.7, //#8 End Speed
                gravity_accel : 0.03, //#9 Gravity Acceleration
                gravity_speed : 0.6, //#10 Gravity Max Speed
                angle_extra : 15.0, //#11 Angle stuff but unknown what this is for
                angle_more_speed : -25.0, //#12 Angle to gain more speed
                down_speed_add : 0.03, //#13 Max added speed gained aiming downward
                unknown : 0.15, //#14 Unknown
                radial_stick : 0.25, //#15 Radial Stick Sensitivity
                up_angle_accel : 0.55, //#16 Upward angular acceleration
                down_angle_accel : 0.75, //#17 Downward angular acceleration
                max_angle_speed : 7.0, //#18 Maximum angular speed
                add_angle_speed : 1.0 //#19 Added angular speed for when stick is center
            };
        }
    }
}