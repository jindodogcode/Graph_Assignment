mod canvas_state;
mod city;
mod consts;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use graph_lib::graph::Graph;
use graph_lib::make_graph;

use canvas_state::CanvasState;
use city::{City, Conn};
use consts::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    init_panic_hook();

    // Get DOM references
    let window = web_sys::window().unwrap();
    let window = Rc::new(window);
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let canvas = Rc::new(canvas);
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let context = Rc::new(context);

    // Assign canvas dimensions
    let win_width = window.inner_width().unwrap().as_f64().unwrap();
    let canvas_width = win_width * (2.0 / 3.0);
    canvas
        .set_attribute("width", &canvas_width.to_string())
        .unwrap();
    let canvas_height = win_width / 3.2;
    canvas
        .set_attribute("height", &canvas_height.to_string())
        .unwrap();

    let graph = make_graph();
    let (cities, connections) = map_nodes(&graph, canvas_height, canvas_width);
    let canvas_state = Rc::new(RefCell::new(CanvasState::new(cities, connections)));

    // Draw initial canvas
    canvas_state.borrow().draw(&canvas, &context);

    // Create tooltips
    setup_tooltips(
        &document,
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::clone(&canvas_state),
    )?;

    // Make cities list
    make_cities_list(
        &document,
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::clone(&canvas_state),
    )?;

    let window_resize_listener = make_window_resize_listener(
        Rc::clone(&window),
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::new(graph),
        Rc::clone(&canvas_state),
    );
    window.add_event_listener_with_callback(
        "resize",
        window_resize_listener.as_ref().unchecked_ref(),
    )?;
    window_resize_listener.forget();

    Ok(())
}

/// Maps coordinates stored in the Graph to coordinates on the canvas
fn map_nodes(graph: &Graph, height: f64, width: f64) -> (HashMap<String, City>, HashSet<Conn>) {
    let height_pad = height * 0.1;
    let width_pad = width * 0.1;
    let height = height - height_pad;
    let width = width - width_pad;
    let height_ratio = height / (US_NORTH - US_SOUTH);
    let height_offset = US_NORTH;
    let width_ratio = width / (US_EAST - US_WEST);
    let width_offset = US_WEST;

    let mut mapped_nodes: HashMap<String, City> = HashMap::with_capacity(graph.nodes().len());
    let mut connections: HashSet<Conn> = HashSet::new();

    for (id, node) in graph.nodes().iter() {
        let row = (node.point().row() as f64 - height_offset) * height_ratio * -1.0;
        let row = (row + (height_pad / 2.0)).round();
        let col = (node.point().col() as f64 - width_offset) * width_ratio;
        let col = (col + (width_pad / 2.0)).round();

        mapped_nodes.insert(id.clone(), City::new(col, row, DOT_RADIUS));

        for (dest, _) in node.edges().iter() {
            let conn = Conn(dest.to_owned(), id.to_owned());
            if connections.contains(&conn) {
                continue;
            }
            let Conn(dest, id) = conn;
            connections.insert(Conn(id, dest));
        }
    }
    (mapped_nodes, connections)
}

/// Better wasm errors.
fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

fn setup_tooltips(
    document: &web_sys::Document,
    canvas: Rc<web_sys::HtmlCanvasElement>,
    context: Rc<web_sys::CanvasRenderingContext2d>,
    canvas_state: Rc<RefCell<CanvasState>>,
) -> Result<(), JsValue> {
    let listen_canvas = Rc::clone(&canvas);

    let tip_canvas = document.get_element_by_id("tip-canvas").unwrap();
    let tip_canvas: web_sys::HtmlCanvasElement = tip_canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let tip_context = tip_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    tip_context.set_font("12px serif");

    // Tooltip handler
    let tooltip_listener = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let mouse_x = event.offset_x() as f64;
        let mouse_y = event.offset_y() as f64;

        let mut name_reminder: Option<String> = None;

        for (name, city) in canvas_state.borrow().cities().iter() {
            let dx = mouse_x - city.x();
            let dy = mouse_y - city.y();

            if dx * dx + dy * dy < (city.radius() + 1.0).powf(2.0) {
                name_reminder = Some(name.to_owned());

                tip_context.clear_rect(
                    0.0,
                    0.0,
                    tip_canvas.width().into(),
                    tip_canvas.height().into(),
                );

                let tip_text = format!("{} {} {}", name, city.x(), city.y());
                let text_width = tip_context.measure_text(&tip_text).unwrap();
                let text_width = text_width.width() + 6.0;
                let x_offset = listen_canvas.offset_left() - (text_width / 2.0) as i32;
                let y_offset = listen_canvas.offset_top() - 30;
                let x_str = format!("{}px", city.x() as i64 + x_offset as i64);
                let y_str = format!("{}px", city.y() as i64 + y_offset as i64);
                tip_canvas.style().set_property("display", "block").unwrap();
                tip_canvas.style().set_property("left", &x_str).unwrap();
                tip_canvas.style().set_property("top", &y_str).unwrap();
                tip_canvas
                    .set_attribute("width", &text_width.to_string())
                    .unwrap();
                tip_context.fill_text(&tip_text, 3.0, 15.0).unwrap();

                break;
            }
        }
        if let Some(name) = name_reminder {
            canvas_state.borrow_mut().set_hovered(Some(name));
            canvas_state.borrow().draw(&listen_canvas, &context);
        } else {
            tip_canvas.style().set_property("display", "none").unwrap();
            canvas_state.borrow_mut().set_hovered(None);
            canvas_state.borrow().draw(&listen_canvas, &context);
        }
    }) as Box<dyn FnMut(_)>);

    canvas
        .add_event_listener_with_callback("mousemove", tooltip_listener.as_ref().unchecked_ref())
        .unwrap();
    tooltip_listener.forget();

    Ok(())
}

/// Creates list of cities on left side of windows and adds a mouseover listener.
fn make_cities_list(
    document: &web_sys::Document,
    canvas: Rc<web_sys::HtmlCanvasElement>,
    context: Rc<web_sys::CanvasRenderingContext2d>,
    canvas_state: Rc<RefCell<CanvasState>>,
) -> Result<(), JsValue> {
    let listen_in_state = Rc::clone(&canvas_state);
    let listen_out_canvas = Rc::clone(&canvas);
    let listen_out_context = Rc::clone(&context);
    let listen_out_state = Rc::clone(&canvas_state);

    // mouseover listener for list
    let list_on_listener = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let target = event.target().unwrap();
        let target = target
            .dyn_into::<web_sys::HtmlElement>()
            .map_err(|_| ())
            .unwrap();
        target
            .style()
            .set_property("color", "#09c")
            .expect("element should have css");
        let name = target.inner_html();

        listen_in_state.borrow_mut().set_hovered(Some(name));
        listen_in_state.borrow().draw(&canvas, &context);
    }) as Box<dyn FnMut(_)>);

    // mouseout listener for list
    let list_out_listener = Closure::wrap(Box::new(move |event: web_sys::Event| {
        let target = event.target().unwrap();
        let target = target
            .dyn_into::<web_sys::HtmlElement>()
            .map_err(|_| ())
            .unwrap();
        target
            .style()
            .set_property("color", TEXT_COLOR)
            .expect("element should have css");

        listen_out_state.borrow_mut().set_hovered(None);
        listen_out_state
            .borrow()
            .draw(&listen_out_canvas, &listen_out_context);
    }) as Box<dyn FnMut(_)>);

    let html_cities_list = document.create_element("ul")?;
    html_cities_list.set_attribute("id", "cities-list")?;
    let mut sorted_cities: Vec<String> = canvas_state
        .borrow()
        .cities()
        .iter()
        .map(|(name, _)| name.clone())
        .collect();
    sorted_cities.sort_unstable();
    for city in sorted_cities.iter() {
        let html_city = document.create_element("li")?;
        let html_city = html_city
            .dyn_into::<web_sys::HtmlElement>()
            .map_err(|_| ())
            .unwrap();
        html_city.set_inner_html(&city);
        html_city.set_attribute("class", "city-list-item")?;
        html_city.add_event_listener_with_callback(
            "mouseover",
            list_on_listener.as_ref().unchecked_ref(),
        )?;
        html_city.add_event_listener_with_callback(
            "mouseout",
            list_out_listener.as_ref().unchecked_ref(),
        )?;

        html_cities_list.append_child(&html_city)?;
    }

    list_on_listener.forget();
    list_out_listener.forget();

    let left_sidebar = document.get_element_by_id("left-list-wrapper").unwrap();
    let left_para = document.create_element("p")?;
    left_para.set_attribute("id", "cities-list-title")?;
    left_para.set_inner_html("Cities:");
    left_sidebar.append_child(&left_para)?;
    left_sidebar.append_child(&html_cities_list)?;

    Ok(())
}

/// Adjust canvas size to window size.
fn make_window_resize_listener(
    window: Rc<web_sys::Window>,
    canvas: Rc<web_sys::HtmlCanvasElement>,
    context: Rc<web_sys::CanvasRenderingContext2d>,
    graph: Rc<Graph>,
    canvas_state: Rc<RefCell<CanvasState>>,
) -> Closure<(dyn FnMut(web_sys::Event) + 'static)> {
    Closure::wrap(Box::new(move |_| {
        let win_width = window.inner_width().unwrap().as_f64().unwrap();
        let canvas_width = win_width * (2.0 / 3.0);
        canvas
            .set_attribute("width", &canvas_width.to_string())
            .unwrap();
        let canvas_height = win_width / 3.2;
        canvas
            .set_attribute("height", &canvas_height.to_string())
            .unwrap();

        let (cities, connections) = map_nodes(&graph, canvas_height, canvas_width);
        canvas_state.borrow_mut().set_cities(cities);
        canvas_state.borrow_mut().set_connections(connections);

        canvas_state.borrow().draw(&canvas, &context);
    }) as Box<dyn FnMut(_)>)
}
