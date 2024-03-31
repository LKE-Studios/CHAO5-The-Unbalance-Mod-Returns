mod robot;
mod robot_beam;
mod robot_mainlaser;

pub fn install() {
    robot::install();   
    robot_beam::install();
    robot_mainlaser::install();    
}