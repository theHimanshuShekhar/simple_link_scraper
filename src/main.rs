use petgraph::Directed;
use petgraph::graph::Graph;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("simple_web_scraper!");

    let start_url = "https://www.rust-lang.org";

    // Queue of URLs to visit
    let mut queue = VecDeque::new();
    queue.push_back(start_url.to_string());

    // Set to keep track of visited URLs
    let mut visited = HashSet::new();
    visited.insert(start_url.to_string());

    // Create a graph of URLs
    let mut graph = Graph::<String, (), Directed>::new();
    let mut url_to_index = HashMap::new();

    // Add start URL to graph
    let start_index = graph.add_node(start_url.to_string());
    url_to_index.insert(start_url.to_string(), start_index);

    // loop until the queue is empty
    while let Some(url) = queue.pop_front() {
        // Exit out of loop if we have visited 1000 URLs
        if visited.len() >= 1000 {
            break;
        }
        println!("Visiting: {}", url);
        let body = reqwest::get(&url).await?.text().await?;
        let document = scraper::Html::parse_document(&body);
        let selector = scraper::Selector::parse("a").unwrap();

        let current_index = *url_to_index.get(&url).unwrap();

        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if href.starts_with("http") && !visited.contains(href) {
                    println!("New link found: {href}");
                    visited.insert(href.to_string());
                    queue.push_back(href.to_string());

                    // Add the link to the graph
                    let target_index = if let Some(&idx) = url_to_index.get(href) {
                        idx
                    } else {
                        let idx = graph.add_node(href.to_string());
                        url_to_index.insert(href.to_string(), idx);
                        idx
                    };

                    // Add edge from current URL to found URL
                    graph.add_edge(current_index, target_index, ());
                }
            }
        }
    }

    // Print graph statistics
    println!(
        "Graph contains {} nodes and {} edges",
        graph.node_count(),
        graph.edge_count()
    );
    println!("Total unique URLs visited: {}", visited.len());

    // Write graph to file
    let mut file = std::fs::File::create("graph.dot")?;
    write!(file, "{:?}", petgraph::dot::Dot::new(&graph))?; // Write graph to file

    Ok(())
}
