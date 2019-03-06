use std::string::ToString;

pub struct Stroke {
    pub rgb: String,
    pub alpha: f64,
    pub x: f64,
    pub y: f64,
    pub size: f64,
}

impl ToString for Stroke {
    fn to_string(&self) -> String {
        format!("{{\"rgb\":{:?},\"alpha\":{:?},\"x\":{:?},\"y\":{:?},\"size\":{:?}}}", self.rgb, self.alpha, self.x, self.y, self.size)
    }
}

// {"rgb":"#000000","alpha":1,"x":50,"y":50,"size":3}