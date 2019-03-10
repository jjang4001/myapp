use std::string::ToString;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Stroke {
    pub rgb: String,
    pub alpha: f64,
    pub x: f64,
    pub y: f64,
    pub size: f64,
    pub endpoint: String,
}

// impl Stroke {
//     pub fn new(stroke_string: &str) -> Stroke {

//     } 
// }

impl ToString for Stroke {
    fn to_string(&self) -> String {
        format!("{{\"rgb\":{:?},\"alpha\":{:?},\"x\":{:?},\"y\":{:?},\"size\":{:?},\"endpoint\":{:?}}}", self.rgb, self.alpha, self.x, self.y, self.size, self.endpoint)
    }
}

// {"rgb":"#000000","alpha":1,"x":50,"y":50,"size":3}