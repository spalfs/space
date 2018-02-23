use mass::Mass;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Ship {
    name        : String,
    location    : (isize, isize, isize),
    r           : f64,
}

impl Ship {
    pub fn range(&self) -> f64 {
        self.r
    }
}

impl Mass for Ship {
    fn new(name : &str, location : (isize, isize, isize)) -> Ship {
        Ship {
            name        : String::from(name),
            location    : location,
            r           : 100.0,
        }
    }

    fn name(&self) -> &String {
        &self.name
    }

    fn location(&self) -> (isize, isize, isize) {
        self.location
    }

    fn set_location(&mut self, location : (isize, isize, isize)) {
        self.location = location;
    }

    fn serialize(&self) ->String {
        serde_json::to_string(self).unwrap()
    }

    fn deserialize(&mut self, data : &str) {
        //self = serde_json::from_str(data).unwrap();
    }
}
