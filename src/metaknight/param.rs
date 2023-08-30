use crate::imports::BuildImports::*;

pub struct SpecialAirDiveParams {
    pub stick_angle_mul : f32, //Stick Angle Multiplier
    pub angle_max_left : f32, //Max Leftward Angle
    pub angle_max_right : f32, //Max Rightward Angle
    pub max_dive_speed : f32, //Max Rightward Angle
}

impl SpecialAirDiveParams {
    pub fn get() -> SpecialAirDiveParams {
        SpecialAirDiveParams {
            stick_angle_mul : 7.5,
            angle_max_left : -30.0,
            angle_max_right : 30.0,
            max_dive_speed : 3.4,
        }
    }
}