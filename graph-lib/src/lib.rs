mod adj_list;
mod adj_matrix;
pub mod graph;

use crate::graph::{point::Point, Graph};

pub fn make_graph() -> Graph {
    let mut cities = Graph::with_capacity(15);

    cities.add_node("Boston, MA", Point::new(42_21_29, -71_03_49));
    cities.add_node("New York, NY", Point::new(40_42_46, -74_00_21));
    cities.add_node("Washington, DC", Point::new(38_54_17, -77_00_59));
    cities.add_node("Atlanta, GA", Point::new(33_45_18, -84_23_24));
    cities.add_node("Miami, FL", Point::new(25_46_31, -80_12_32));
    cities.add_node("Detroit, MI", Point::new(42_19_53, -83_02_45));
    cities.add_node("Chicago, IL", Point::new(41_50_13, -87_41_05));
    cities.add_node("Houston, TX", Point::new(29_45_46, -95_22_59));
    cities.add_node("Dallas, TX", Point::new(32_47_00, -96_48_00));
    cities.add_node("Denver, CO", Point::new(39_45_43, -104_52_52));
    cities.add_node("Phoenix, AZ", Point::new(33_27_00, -112_04_00));
    cities.add_node("Las Vegas, NV", Point::new(36_10_30, -115_08_11));
    cities.add_node("Los Angeles, CA", Point::new(34_03_00, -118_15_00));
    cities.add_node("Seattle, WA", Point::new(47_36_35, -122_19_59));
    cities.add_node("San Francisco, CA", Point::new(37_47_00, -122_25_00));

    cities.add_edge("Boston, MA", "New York, NY");
    cities.add_edge("Boston, MA", "Detroit, MI");
    cities.add_edge("New York, NY", "Washington, DC");
    cities.add_edge("Washington, DC", "Atlanta, GA");
    cities.add_edge("New York, NY", "Detroit, MI");
    cities.add_edge("New York, NY", "Chicago, IL");
    cities.add_edge("New York, NY", "Miami, FL");
    cities.add_edge("Washington, DC", "Chicago, IL");
    cities.add_edge("Atlanta, GA", "Dallas, TX");
    cities.add_edge("Atlanta, GA", "Miami, FL");
    cities.add_edge("Atlanta, GA", "Houston, TX");
    cities.add_edge("Miami, FL", "Dallas, TX");
    cities.add_edge("Miami, FL", "Houston, TX");
    cities.add_edge("Detroit, MI", "Chicago, IL");
    cities.add_edge("Detroit, MI", "Seattle, WA");
    cities.add_edge("Chicago, IL", "Dallas, TX");
    cities.add_edge("Chicago, IL", "Denver, CO");
    cities.add_edge("Dallas, TX", "Houston, TX");
    cities.add_edge("Dallas, TX", "Phoenix, AZ");
    cities.add_edge("Denver, CO", "Phoenix, AZ");
    cities.add_edge("Denver, CO", "Seattle, WA");
    cities.add_edge("Denver, CO", "Las Vegas, NV");
    cities.add_edge("Denver, CO", "San Francisco, CA");
    cities.add_edge("Denver, CO", "Los Angeles, CA");
    cities.add_edge("Las Vegas, NV", "Phoenix, AZ");
    cities.add_edge("Las Vegas, NV", "Dallas, TX");
    cities.add_edge("Las Vegas, NV", "Los Angeles, CA");
    cities.add_edge("Phoenix, AZ", "Los Angeles, CA");
    cities.add_edge("San Francisco, CA", "Seattle, WA");
    cities.add_edge("San Francisco, CA", "Los Angeles, CA");

    cities
}
