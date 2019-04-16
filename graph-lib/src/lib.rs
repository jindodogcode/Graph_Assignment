mod adj_list;
mod adj_matrix;
pub mod graph;

use graph::{point::Point, Graph};

pub fn make_graph() -> Graph {
    let mut cities = Graph::with_capacity(15);

    cities.add_node(
        "Boston, MA",
        Point::new(gstring_parse("42°21'29\""), gstring_parse("-71°03'49\"")),
    );
    cities.add_node(
        "New York, NY",
        Point::new(gstring_parse("40°42'46\""), gstring_parse("-74°00'21\"")),
    );
    cities.add_node(
        "Washington, DC",
        Point::new(gstring_parse("38°54'17\""), gstring_parse("-77°00'59\"")),
    );
    cities.add_node(
        "Atlanta, GA",
        Point::new(gstring_parse("33°45'18\""), gstring_parse("-84°23'24\"")),
    );
    cities.add_node(
        "Miami, FL",
        Point::new(gstring_parse("25°46'31\""), gstring_parse("-80°12'32\"")),
    );
    cities.add_node(
        "Detroit, MI",
        Point::new(gstring_parse("42°19'53\""), gstring_parse("-83°02'45\"")),
    );
    cities.add_node(
        "Chicago, IL",
        Point::new(gstring_parse("41°50'13\""), gstring_parse("-87°41'05\"")),
    );
    cities.add_node(
        "Houston, TX",
        Point::new(gstring_parse("29°45'46\""), gstring_parse("-95°22'59\"")),
    );
    cities.add_node(
        "Dallas, TX",
        Point::new(gstring_parse("32°47'00\""), gstring_parse("-96°48'00\"")),
    );
    cities.add_node(
        "Denver, CO",
        Point::new(gstring_parse("39°45'43\""), gstring_parse("-104°52'52\"")),
    );
    cities.add_node(
        "Phoenix, AZ",
        Point::new(gstring_parse("33°27'00\""), gstring_parse("-112°04'00\"")),
    );
    cities.add_node(
        "Las Vegas, NV",
        Point::new(gstring_parse("36°10'30\""), gstring_parse("-115°08'11\"")),
    );
    cities.add_node(
        "Los Angeles, CA",
        Point::new(gstring_parse("34°03'00\""), gstring_parse("-118°15'00\"")),
    );
    cities.add_node(
        "Seattle, WA",
        Point::new(gstring_parse("47°36'35\""), gstring_parse("-122°19'59\"")),
    );
    cities.add_node(
        "San Francisco, CA",
        Point::new(gstring_parse("37°47'00\""), gstring_parse("-122°25'00\"")),
    );

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

pub fn gstring_parse(geo_string: &str) -> f64 {
    let (degrees, ms) = geo_string.split_at(geo_string.find('°').unwrap());
    let (minutes, seconds) = ms.split_at(ms.find('\'').unwrap());
    let degrees: f64 = degrees.parse().unwrap();
    let mut minutes: f64 = minutes.trim_start_matches('°').parse().unwrap();
    let mut seconds: f64 = seconds
        .trim_start_matches('\'')
        .trim_end_matches('"')
        .parse()
        .unwrap();
    minutes = minutes / 60.0;
    seconds = seconds / 3600.0;
    degrees + minutes + seconds
}
