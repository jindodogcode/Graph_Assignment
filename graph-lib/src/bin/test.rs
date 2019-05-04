fn main() {
    let graph = graph_lib::make_graph();
    let search_results = match graph.depth_first_search("Seattle, WA", "Miami, FL") {
        Ok(results) => results,
        Err(e) => {
            eprintln!("{}", e);
            return;
        }
    };

    match search_results {
        Some(result) => {
            for (id, dist) in result.iter() {
                println!("{}: {}", id, dist);
            }
        }
        None => {
            println!("None");
        }
    }

    let bfs_results = match graph.breadth_first_search("Seattle, WA", "Miami, FL") {
        Ok(results) => results,
        Err(e) => {
            eprintln!("{}J", e);
            return;
        }
    };

    match bfs_results {
        Some(results) => {
            for (id, dist) in results.iter() {
                println!("{}: {}", id, dist);
            }
        }
        None => {
            println!("None");
        }
    }

    let dij_results = match graph.shortest_path("Seattle, WA", "Miami, FL") {
        Ok(results) => results,
        Err(e) => {
            eprintln!("{}J", e);
            return;
        }
    };

    match dij_results {
        Some(results) => {
            for (id, dist) in results.iter() {
                println!("{}: {}", id, dist);
            }
        }
        None => {
            println!("None");
        }
    }
}
