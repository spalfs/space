use std::time::SystemTime;
use std::collections::HashMap;

use mass::Mass;
use math::distance;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum NavigationStatus {
    None,
    Targeting,
    Targeted,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Navigation {
    pub range       : f64,
    pub status      : NavigationStatus,
    pub target_name : Option<String>,
    time            : u64,
    start           : Option<SystemTime>,
}

impl Navigation {
    pub fn new() -> Navigation {
        Navigation {
            target_name : None,
            range       : 100.0,
            status      : NavigationStatus::None,
            time        : 3,
            start       : None,
        }
    }

    pub fn process(&mut self) {
        match self.start.clone() {
            Some(timer) => {
                if timer.elapsed().unwrap().as_secs() > self.time {
                    self.status = NavigationStatus::Targeted;
                    self.start = None;
                }
            }
            _ => (),
        }
    }

    pub fn give_target(&mut self, target_name : String) {
        self.start = Some(SystemTime::now());
        self.status = NavigationStatus::Targeting;
        self.target_name = Some(target_name);
    }

    pub fn verify_target(&mut self, ship_position : (f64, f64, f64), masses : &HashMap<String, Mass>) {
        match self.target_name.clone() {
            Some(name) => {
                let target = masses.get(&name).unwrap();
                if distance(target.position, ship_position) > self.range {
                    self.target_name = None;
                    self.status = NavigationStatus::None;
                }
            }
            _ => (),
        }
    }
}