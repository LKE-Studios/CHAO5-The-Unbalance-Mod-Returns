use crate::imports::BuildImports::*;

pub struct SpecialHiFlyParams {
    pub pass_mul : f32, //Grounded Height multiplier
    pub lr_stick_x : f32, //H speed multiplier on startup
    pub dir_stick_x : f32,
    pub dir_mul : f32, //Ability to angle (degrees)
    pub accel_y : f32,
    pub start_x_mul : f32,
    pub fall_x : f32,
    pub motion_rate_min : f32, //Motion rate when stick y is not held
    pub motion_rate_max : f32, //Motion rate multiplier stick y is held
    pub gravity_speed : f32, //Gravity speed
    pub air_decel_y : f32, //V Deceleration
    pub air_accel_x : f32, //H Acceleration
    pub air_accel_y : f32, //V Acceleration
    pub speed_x_max : f32, //Max H Speed
    pub speed_y_max : f32, // Max V Speed
    pub fly_end_frame : i32, //Max Flight Time
}

impl SpecialHiFlyParams {
    pub fn get() -> SpecialHiFlyParams {
        SpecialHiFlyParams {
            pass_mul : 1.5,
            lr_stick_x : 0.25,
            dir_stick_x : 0.625,
            dir_mul : 30.0,
            accel_y : 0.5,
            start_x_mul : 0.6,
            fall_x : 1.0,
            motion_rate_min : 0.35,
            motion_rate_max : 2.6, 
            gravity_speed : 0.3,
            air_decel_y : 0.01, 
            air_accel_x : 0.03,
            air_accel_y : 0.05,
            speed_x_max : 1.6,
            speed_y_max : 2.1,
            fly_end_frame : 240
        }
    }
}