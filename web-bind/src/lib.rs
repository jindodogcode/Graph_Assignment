mod city;

use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use city::{City, Conn};
use graph_lib::graph::Graph;
use graph_lib::make_graph;

const CONN_COLOR: &str = "#f33";
const CITY_COLOR: &str = "#000";
const SPECIAL_CONN_COLOR: &str = "#3f7";
const SPECIAL_CITY_COLOR: &str = "#37f";
const TEXT_COLOR: &str = "#ddd";

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    init_panic_hook();

    let graph = make_graph();

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

    // let window_resize_listener =
    //     make_window_resize_listener(Rc::clone(&window), Rc::clone(&canvas));
    // window.add_event_listener_with_callback(
    //     "resize",
    //     window_resize_listener.as_ref().unchecked_ref(),
    // )?;
    // window_resize_listener.forget();

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

    let (cities, connections) = map_nodes(&graph, canvas_height, canvas_width);
    let cities = Rc::new(cities);
    let connections = Rc::new(connections);

    // Draw connections
    draw_all_connections(&cities, &connections, &context, &CONN_COLOR.into());

    // Draw cities
    draw_all_cities(&cities, &context, &CITY_COLOR.into());

    // Create tooltips
    setup_tooltips(&document, Rc::clone(&canvas), Rc::clone(&cities))?;

    // Make cities list
    make_cities_list(
        &document,
        &graph,
        Rc::clone(&cities),
        Rc::clone(&connections),
        Rc::clone(&canvas),
        Rc::clone(&context),
    )?;

    Ok(())
}

fn draw_all_cities(
    cities: &HashMap<String, City>,
    context: &web_sys::CanvasRenderingContext2d,
    color: &JsValue,
) {
    for (_, city) in cities.iter() {
        city.draw(&context, color);
    }
}
fn draw_all_connections(
    cities: &HashMap<String, City>,
    connections: &HashSet<Conn>,
    context: &web_sys::CanvasRenderingContext2d,
    color: &JsValue,
) {
    for Conn(src, dest) in connections.iter() {
        let src_x = cities[src].x();
        let src_y = cities[src].y();
        let dest_x = cities[dest].x();
        let dest_y = cities[dest].y();

        context.set_stroke_style(color);
        context.begin_path();

        context.move_to(src_x, src_y);
        context.line_to(dest_x, dest_y);

        context.close_path();
        context.stroke();
    }
}

fn redraw_with_color(
    cities: &HashMap<String, City>,
    connections: &HashSet<Conn>,
    select_city: &str,
    context: &web_sys::CanvasRenderingContext2d,
    color: &JsValue,
    special_color: &JsValue,
    city_color: &JsValue,
    special_city_color: &JsValue,
) {
    for Conn(src, dest) in connections.iter() {
        if select_city == src || select_city == dest {
            context.set_stroke_style(special_color);
        } else {
            context.set_stroke_style(color);
        }
        let src_x = cities[src].x();
        let src_y = cities[src].y();
        let dest_x = cities[dest].x();
        let dest_y = cities[dest].y();

        context.begin_path();
        context.move_to(src_x, src_y);
        context.line_to(dest_x, dest_y);
        context.close_path();
        context.stroke();
    }

    for (name, city) in cities.iter() {
        if select_city == name {
            city.draw(context, special_city_color);
        } else {
            city.draw(context, city_color);
        }
    }
}

fn map_nodes(graph: &Graph, height: f64, width: f64) -> (HashMap<String, City>, HashSet<Conn>) {
    const US_NORTH: f64 = 49_23_04.0;
    const US_SOUTH: f64 = 24_26_80.0;
    const US_WEST: f64 = -124_47_10.0;
    const US_EAST: f64 = -66_56_59.0;
    const DOT_RADIUS: f64 = 5.0;

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

fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

fn setup_tooltips(
    document: &web_sys::Document,
    canvas: Rc<web_sys::HtmlCanvasElement>,
    cities: Rc<HashMap<String, City>>,
) -> Result<(), JsValue> {
    let canvas_2 = Rc::clone(&canvas);

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

        let mut hit = false;

        for (name, city) in cities.iter() {
            let dx = mouse_x - city.x();
            let dy = mouse_y - city.y();

            if dx * dx + dy * dy < (city.radius() + 1.0).powf(2.0) {
                hit = true;
                let tip_text = format!("{} {} {}", name, city.x(), city.y());
                let text_width = tip_context.measure_text(&tip_text).unwrap();
                let text_width = text_width.width() + 6.0;
                let x_offset = canvas.offset_left() - (text_width / 2.0) as i32;
                let y_offset = canvas.offset_top() - 30;
                let x_str = format!("{}px", mouse_x as i64 + x_offset as i64);
                let y_str = format!("{}px", mouse_y as i64 + y_offset as i64);
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
        if !hit {
            tip_canvas.style().set_property("display", "none").unwrap();
            tip_context.clear_rect(
                0.0,
                0.0,
                tip_canvas.width().into(),
                tip_canvas.height().into(),
            );
        }
    }) as Box<dyn FnMut(_)>);

    canvas_2
        .add_event_listener_with_callback("mousemove", tooltip_listener.as_ref().unchecked_ref())
        .unwrap();
    tooltip_listener.forget();

    Ok(())
}

fn make_cities_list(
    document: &web_sys::Document,
    graph: &Graph,
    cities: Rc<HashMap<String, City>>,
    connections: Rc<HashSet<Conn>>,
    canvas: Rc<web_sys::HtmlCanvasElement>,
    context: Rc<web_sys::CanvasRenderingContext2d>,
) -> Result<(), JsValue> {
    let listen_out_context = Rc::clone(&context);
    let listen_out_connections = Rc::clone(&connections);
    let listen_out_canvas = Rc::clone(&canvas);
    let listen_cities = Rc::clone(&cities);

    // Mouse on listener for list
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

        context.save();
        context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
        redraw_with_color(
            &listen_cities,
            &connections,
            &name,
            &context,
            &CONN_COLOR.into(),
            &SPECIAL_CONN_COLOR.into(),
            &CITY_COLOR.into(),
            &SPECIAL_CITY_COLOR.into(),
        );
    }) as Box<dyn FnMut(_)>);

    // Mouse out listener for list
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

        listen_out_context.clear_rect(
            0.0,
            0.0,
            listen_out_canvas.width() as f64,
            listen_out_canvas.height() as f64,
        );
        draw_all_connections(
            &cities,
            &listen_out_connections,
            &listen_out_context,
            &CONN_COLOR.into(),
        );
        draw_all_cities(&cities, &listen_out_context, &CITY_COLOR.into());
    }) as Box<dyn FnMut(_)>);

    let html_cities_list = document.create_element("ul")?;
    html_cities_list.set_attribute("id", "cities-list")?;
    let mut sorted_cities: Vec<String> =
        graph.nodes().iter().map(|(name, _)| name.clone()).collect();
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
    let left_sidebar = document.get_element_by_id("left-list-wrapper").unwrap();
    let left_para = document.create_element("p")?;
    left_para.set_attribute("id", "cities-list-title")?;
    left_para.set_inner_html("Cities:");
    left_sidebar.append_child(&left_para)?;
    left_sidebar.append_child(&html_cities_list)?;

    list_on_listener.forget();
    list_out_listener.forget();

    Ok(())
}

fn make_window_resize_listener(
    window: Rc<web_sys::Window>,
    canvas: Rc<web_sys::HtmlCanvasElement>,
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
    }) as Box<dyn FnMut(_)>)
}
