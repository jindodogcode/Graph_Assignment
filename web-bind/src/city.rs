use std::f64::consts::PI;

use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

#[derive(Debug, Clone, PartialEq)]
pub struct City {
    x: f64,
    y: f64,
    radius: f64,
}

impl City {
    pub fn new(x: f64, y: f64, radius: f64) -> City {
        City { x, y, radius }
    }
}

impl City {
    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn draw(&self, context: &CanvasRenderingContext2d, color: &JsValue) {
        context.set_fill_style(color);
        context.begin_path();
        context.move_to(self.x, self.y);
        context
            .arc(self.x, self.y, self.radius, 0.0, PI * 2.0)
            .unwrap();
        context.close_path();
        context.fill();
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Conn(pub String, pub String);
