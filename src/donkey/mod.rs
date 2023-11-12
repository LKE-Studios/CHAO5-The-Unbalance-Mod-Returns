mod donkey;

pub fn install() {
    donkey::install(&mut Agent::new("donkey")); 
}