mod canvas_state;
mod city;
mod consts;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

use graph_lib::graph::search::{Search, Status};
use graph_lib::graph::Graph;
use graph_lib::{gstring_parse, make_graph};

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
    let document = Rc::new(document);
    let canvas = document
        .get_element_by_id("canvas")
        .expect("document should contain canvas element");
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
    let graph = Rc::new(graph);
    let (cities, connections) = map_nodes(&graph, canvas_height, canvas_width);
    let canvas_state = Rc::new(RefCell::new(CanvasState::new(cities, connections)));

    // Make color key
    make_color_key(&document)?;

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
        Rc::clone(&graph),
        Rc::clone(&canvas_state),
    );
    window.add_event_listener_with_callback(
        "resize",
        window_resize_listener.as_ref().unchecked_ref(),
    )?;

    // leaks memory
    window_resize_listener.forget();

    let search_button = document
        .get_element_by_id("search-btn")
        .expect("document should contain search-btn element");
    let search_button = search_button
        .dyn_into::<web_sys::HtmlButtonElement>()
        .map_err(|_| ())
        .unwrap();
    let search_click_listener = make_search_button_listener(
        Rc::clone(&window),
        Rc::clone(&document),
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::clone(&graph),
        Rc::clone(&canvas_state),
    );
    search_button.add_event_listener_with_callback(
        "click",
        search_click_listener.as_ref().unchecked_ref(),
    )?;
    search_click_listener.forget();

    let reset_button = document
        .get_element_by_id("reset-btn")
        .expect("document should contain reset-btn element");
    let reset_button = reset_button
        .dyn_into::<web_sys::HtmlButtonElement>()
        .map_err(|_| ())
        .unwrap();
    let reset_button_click_listener = make_reset_button_listener(
        Rc::clone(&canvas),
        Rc::clone(&context),
        Rc::clone(&canvas_state),
    );
    reset_button.add_event_listener_with_callback(
        "click",
        reset_button_click_listener.as_ref().unchecked_ref(),
    )?;

    // leaks memory
    reset_button_click_listener.forget();

    Ok(())
}

/// Maps coordinates stored in the Graph to coordinates on the canvas
fn map_nodes(graph: &Graph, height: f64, width: f64) -> (HashMap<String, City>, HashSet<Conn>) {
    // These are the farthest points of the continential US
    let us_north: f64 = gstring_parse("49°23'04\"");
    let us_south: f64 = gstring_parse("24°26'80\"");
    let us_west: f64 = gstring_parse("-124°47'10\"");
    let us_east: f64 = gstring_parse("-66°56'59\"");

    let height_pad = height * 0.1;
    let width_pad = width * 0.1;
    let height = height - height_pad;
    let width = width - width_pad;
    let height_ratio = height / (us_north - us_south);
    let height_offset = us_north;
    let width_ratio = width / (us_east - us_west);
    let width_offset = us_west;

    let mut mapped_nodes: HashMap<String, City> = HashMap::with_capacity(graph.nodes().len());
    let mut connections: HashSet<Conn> = HashSet::new();

    for (id, node) in graph.nodes().iter() {
        let row = (node.point().row() - height_offset) * height_ratio * -1.0;
        let row = (row + (height_pad / 2.0)).round();
        let col = (node.point().col() - width_offset) * width_ratio;
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

    // Tooltip mouseover listener
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

    // leaks memory
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

    // leaks memory
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

fn make_color_key(document: &web_sys::Document) -> Result<(), JsValue> {
    let color_key_items = document
        .get_element_by_id("color-key-list")
        .expect("document should contain color-key-items element");

    let undiscovered = document.create_element("div")?;
    let undiscovered = undiscovered
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();
    undiscovered.set_attribute("class", "color-key-item")?;
    let undiscovered_color = document.create_element("div")?;
    let undiscovered_color = undiscovered_color
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();
    undiscovered_color.set_attribute("class", "color-block")?;
    undiscovered_color
        .style()
        .set_property("background", consts::CONN_COLOR)?;
    undiscovered.set_inner_html("Undiscovered: ");
    undiscovered.append_child(&undiscovered_color)?;
    color_key_items.append_child(&undiscovered)?;

    let discovered = document.create_element("div")?;
    let discovered = discovered
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();
    discovered.set_attribute("class", "color-key-item")?;
    let discovered_color = document.create_element("div")?;
    let discovered_color = discovered_color
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();
    discovered_color.set_attribute("class", "color-block")?;
    discovered_color
        .style()
        .set_property("background", consts::QUEUED_CONN_COLOR)?;
    discovered.set_inner_html("Discovered: ");
    discovered.append_child(&discovered_color)?;
    color_key_items.append_child(&discovered)?;

    let searched = document.create_element("div")?;
    let searched = searched
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();
    searched.set_attribute("class", "color-key-item")?;
    let searched_color = document.create_element("div")?;
    let searched_color = searched_color
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();
    searched_color.set_attribute("class", "color-block")?;
    searched_color
        .style()
        .set_property("background", consts::SEARCHED_CONN_COLOR)?;
    searched.set_inner_html("Searched: ");
    searched.append_child(&searched_color)?;
    color_key_items.append_child(&searched)?;

    let path = document.create_element("div")?;
    let path = path
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();
    path.set_attribute("class", "color-key-item")?;
    let path_color = document.create_element("div")?;
    let path_color = path_color
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();
    path_color.set_attribute("class", "color-block")?;
    path_color
        .style()
        .set_property("background", consts::PATH_CONN_COLOR)?;
    path.set_inner_html("Path: ");
    path.append_child(&path_color)?;
    color_key_items.append_child(&path)?;

    let current = document.create_element("div")?;
    let current = current
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();
    current.set_attribute("class", "color_key_items")?;
    let current_color = document.create_element("div")?;
    let current_color = current_color
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();
    current_color.set_attribute("class", "color-block")?;
    current_color
        .style()
        .set_property("background", consts::SELECTED_CITY_COLOR)?;
    current.set_inner_html("Current: ");
    current.append_child(&current_color)?;
    color_key_items.append_child(&current)?;

    Ok(())
}

/// Adjust canvas size based on the window size.
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

fn make_search_button_listener(
    window: Rc<web_sys::Window>,
    document: Rc<web_sys::Document>,
    canvas: Rc<web_sys::HtmlCanvasElement>,
    context: Rc<web_sys::CanvasRenderingContext2d>,
    graph: Rc<Graph>,
    canvas_state: Rc<RefCell<CanvasState>>,
) -> Closure<(dyn FnMut(web_sys::Event) + 'static)> {
    let search_type = document
        .get_element_by_id("search-type")
        .expect("search-type should exists in the document");
    let search_type: web_sys::HtmlSelectElement = search_type
        .dyn_into::<web_sys::HtmlSelectElement>()
        .map_err(|_| ())
        .unwrap();

    let src_in = document
        .get_element_by_id("src-in")
        .expect("src-in should exists in the document");
    let src_in: web_sys::HtmlInputElement = src_in
        .dyn_into::<web_sys::HtmlInputElement>()
        .map_err(|_| ())
        .unwrap();

    let dest_in = document
        .get_element_by_id("dest-in")
        .expect("dest-in should exists in the document");
    let dest_in = dest_in
        .dyn_into::<web_sys::HtmlInputElement>()
        .map_err(|_| ())
        .unwrap();

    let performance = window
        .performance()
        .expect("performance should be available");
    let performance = Rc::new(performance);

    let document = Rc::clone(&document);

    Closure::wrap(Box::new(move |_| {
        let src_in_value = src_in.value();
        let dest_in_value = dest_in.value();
        let search_type_value = search_type.value();
        let error_text = document
            .get_element_by_id("search-error-text")
            .expect("document should contain search-error-text element");
        let error_text = error_text
            .dyn_into::<web_sys::HtmlElement>()
            .map_err(|_| ())
            .unwrap();

        if !graph.nodes().contains_key(&src_in_value) || !graph.nodes().contains_key(&dest_in_value)
        {
            error_text
                .style()
                .set_property("display", "inline")
                .unwrap();
            return;
        } else {
            error_text.style().set_property("display", "none").unwrap();
        }

        let search: Box<RefCell<Search>>;

        match search_type_value.as_ref() {
            "dfs" => {
                search = Box::new(RefCell::new(
                    (*graph)
                        .clone()
                        .rc_step_depth_first_search(&src_in_value, &dest_in_value),
                ));
            }
            "bfs" => {
                search = Box::new(RefCell::new(
                    (*graph)
                        .clone()
                        .rc_step_breadth_first_search(&src_in_value, &dest_in_value),
                ));
            }
            // all cases must be matched
            _ => {
                // search has to be initialized
                // TODO: come up with a better sulution for this
                search = Box::new(RefCell::new(
                    (*graph)
                        .clone()
                        .rc_step_depth_first_search(&src_in_value, &dest_in_value),
                ));
            }
        }

        // Variables to move to search closure
        let mut now = 0.0;
        let mut then = performance.now();
        let interval = 1000.0;
        let mut delta = 0.0;

        let search_window = Rc::clone(&window);
        let performance = Rc::clone(&performance);
        let canvas = Rc::clone(&canvas);
        let context = Rc::clone(&context);
        let canvas_state = Rc::clone(&canvas_state);
        canvas_state.borrow_mut().reset();

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let mut end = false;
        let mut found = false;

        // Search closure
        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            now = performance.now();
            delta = now - then;

            if end {
                let _ = f.borrow_mut().take();
                return;
            }

            if found {
                canvas_state.borrow_mut().set_active(None);
                canvas_state.borrow_mut().set_path(
                    search
                        .borrow()
                        .result()
                        .unwrap()
                        .into_iter()
                        .map(|(s, _)| s)
                        .collect(),
                );

                canvas_state.borrow().draw(&canvas, &context);

                end = true;

                then = now - (delta % interval);
                request_animation_frame(Rc::clone(&search_window), f.borrow().as_ref().unwrap());
            } else if delta > interval {
                let status: Status = search.borrow_mut().next();

                // TODO: DELETE ME
                console::log_2(&"Status: ".into(), &status.to_string().into());

                match status {
                    Status::Searching => {
                        // TODO: DELETE ME
                        console::log_2(&"Current: ".into(), &(*search.borrow().current()).into());
                        canvas_state
                            .borrow_mut()
                            .set_active(Some(search.borrow().current().to_owned()));
                        canvas_state.borrow_mut().set_queued(
                            search
                                .borrow()
                                .visible()
                                .iter()
                                .map(|(id, _)| id.to_string())
                                .collect(),
                        );
                        canvas_state.borrow_mut().set_searched(
                            search
                                .borrow()
                                .visited()
                                .iter()
                                .map(|(id, _)| id.to_string())
                                .collect(),
                        );
                        canvas_state.borrow().draw(&canvas, &context);

                        then = now - (delta % interval);
                        request_animation_frame(
                            Rc::clone(&search_window),
                            f.borrow().as_ref().unwrap(),
                        );
                    }
                    Status::Found => {
                        canvas_state
                            .borrow_mut()
                            .set_active(Some(search.borrow().current().to_owned()));
                        canvas_state.borrow_mut().set_queued(
                            search
                                .borrow()
                                .visible()
                                .iter()
                                .map(|(id, _)| id.to_string())
                                .collect(),
                        );
                        canvas_state.borrow_mut().set_searched(
                            search
                                .borrow()
                                .visited()
                                .iter()
                                .map(|(id, _)| id.to_string())
                                .collect(),
                        );
                        canvas_state.borrow().draw(&canvas, &context);

                        found = true;

                        then = now - (delta % interval);
                        request_animation_frame(
                            Rc::clone(&search_window),
                            f.borrow().as_ref().unwrap(),
                        );
                    }

                    Status::NotFound => {
                        end = true;

                        then = now - (delta % interval);
                        request_animation_frame(
                            Rc::clone(&search_window),
                            f.borrow().as_ref().unwrap(),
                        );
                    }
                }
            } else {
                then = now - (delta % interval);
                request_animation_frame(Rc::clone(&search_window), f.borrow().as_ref().unwrap());
            }
        }) as Box<FnMut()>));

        request_animation_frame(Rc::clone(&window), g.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut(_)>)
}

fn request_animation_frame(window: Rc<web_sys::Window>, f: &Closure<FnMut()>) {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register 'requestAnimationFrame' OK");
}

fn make_reset_button_listener(
    canvas: Rc<web_sys::HtmlCanvasElement>,
    context: Rc<web_sys::CanvasRenderingContext2d>,
    canvas_state: Rc<RefCell<CanvasState>>,
) -> Closure<(dyn FnMut(web_sys::Event) + 'static)> {
    Closure::wrap(Box::new(move |_| {
        canvas_state.borrow_mut().reset();
        canvas_state.borrow().draw(&canvas, &context);
    }) as Box<dyn FnMut(_)>)
}
