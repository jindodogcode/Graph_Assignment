fn main() {
    let graph = graph_lib::make_graph();
    let search_results = match graph.depth_first_search("Miami, FL", "Seattle, WA") {
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

    let bfs_results = match graph.breadth_first_search("Miami, FL", "Seattle, WA") {
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
}
