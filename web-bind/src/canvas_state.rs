use std::cell::RefCell;
use std::collections::{HashMap, HashSet};

use crate::city::{City, Conn};
use crate::consts;

pub struct CanvasState {
    cities: HashMap<String, City>,
    connections: HashSet<Conn>,
    hovered: Option<String>,
    queued: HashSet<String>,
    active: Option<String>,
    searched: HashSet<String>,
    changed_since_last_draw: RefCell<bool>,
}

// Associate functions
impl CanvasState {
    pub fn new(cities: HashMap<String, City>, connections: HashSet<Conn>) -> CanvasState {
        CanvasState {
            cities,
            connections,
            hovered: None,
            queued: HashSet::new(),
            active: None,
            searched: HashSet::new(),
            changed_since_last_draw: RefCell::new(true),
        }
    }
}

// Public methods
#[allow(dead_code)]
impl CanvasState {
    pub fn cities(&self) -> &HashMap<String, City> {
        &self.cities
    }

    pub fn set_cities(&mut self, cities: HashMap<String, City>) {
        if self.cities != cities {
            self.cities = cities;
            self.changed_since_last_draw.replace(true);
        }
    }

    pub fn connections(&self) -> &HashSet<Conn> {
        &self.connections
    }

    pub fn set_connections(&mut self, connections: HashSet<Conn>) {
        if self.connections != connections {
            self.connections = connections;
            self.changed_since_last_draw.replace(true);
        }
    }

    pub fn hovered(&self) -> &Option<String> {
        &self.hovered
    }

    pub fn set_hovered(&mut self, hovered: Option<String>) {
        if self.hovered != hovered {
            self.hovered = hovered;
            self.changed_since_last_draw.replace(true);
        }
    }

    pub fn queued(&self) -> &HashSet<String> {
        &self.queued
    }

    pub fn set_queued(&mut self, queued: HashSet<String>) {
        if self.queued != queued {
            self.queued = queued;
            self.changed_since_last_draw.replace(true);
        }
    }

    pub fn active(&self) -> &Option<String> {
        &self.active
    }

    pub fn set_active(&mut self, active: Option<String>) {
        if self.active != active {
            self.active = active;
            self.changed_since_last_draw.replace(true);
        }
    }

    pub fn searched(&self) -> &HashSet<String> {
        &self.searched
    }

    pub fn set_searched(&mut self, searched: HashSet<String>) {
        if self.searched != searched {
            self.searched = searched;
            self.changed_since_last_draw.replace(true);
        }
    }

    pub fn changed_since_last_draw(&self) -> bool {
        *self.changed_since_last_draw.borrow()
    }

    pub fn draw(
        &self,
        canvas: &web_sys::HtmlCanvasElement,
        context: &web_sys::CanvasRenderingContext2d,
    ) {
        // If state hasn't changed, don't redraw
        if !*self.changed_since_last_draw.borrow() {
            return;
        }

        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

        for Conn(src, dest) in self.connections.iter() {
            if self.hovered.is_some()
                && (self.hovered.as_ref().unwrap() == src || self.hovered.as_ref().unwrap() == dest)
            {
                context.set_stroke_style(&consts::HOVERED_CONN_COLOR.into());
            } else if self.active.is_some()
                && (self.active.as_ref().unwrap() == src || self.active.as_ref().unwrap() == dest)
            {
                context.set_stroke_style(&consts::SELECTED_CONN_COLOR.into());
            } else if self.searched.contains(src) || self.searched.contains(dest) {
                context.set_stroke_style(&consts::SEARCHED_CONN_COLOR.into());
            } else if self.queued.contains(src) || self.queued.contains(dest) {
                context.set_stroke_style(&consts::QUEUED_CONN_COLOR.into());
            } else {
                context.set_stroke_style(&consts::CONN_COLOR.into());
            }

            let src_x = self.cities[src].x();
            let src_y = self.cities[src].y();
            let dest_x = self.cities[dest].x();
            let dest_y = self.cities[dest].y();

            context.begin_path();
            context.move_to(src_x, src_y);
            context.line_to(dest_x, dest_y);
            context.close_path();
            context.stroke();
        }

        for (name, city) in self.cities.iter() {
            let color: &str;
            if self.hovered.is_some() && self.hovered.as_ref().unwrap() == name {
                color = &consts::HOVERED_CITY_COLOR;
            } else if self.active.is_some() && self.active.as_ref().unwrap() == name {
                color = &consts::SELECTED_CITY_COLOR;
            } else if self.searched.contains(name) {
                color = &consts::SEARCHED_CITY_COLOR;
            } else if self.queued.contains(name) {
                color = &consts::QUEUED_CITY_COLOR;
            } else {
                color = &consts::CITY_COLOR;
            }

            city.draw(&context, &color.into());
        }

        self.changed_since_last_draw.replace(false);
    }
}
