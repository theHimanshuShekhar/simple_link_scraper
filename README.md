# Simple Link Scraper 🕷️

A Rust-based web crawler that creates a graph visualization of linked web pages, starting from a given URL.

## ✨ Features

- 🔍 Crawls web pages starting from rust-lang.org
- 📊 Creates a directed graph of webpage connections
- 🛑 Limits crawling to 1000 URLs to prevent excessive requests
- 📄 Generates a DOT file for graph visualization
- ⚡ Asynchronous HTTP requests using tokio and reqwest
- 🔎 HTML parsing with scraper crate

## 📦 Dependencies

- `petgraph`: Graph data structure and algorithms
- `reqwest`: HTTP client
- `scraper`: HTML parsing
- `tokio`: Async runtime
- `url`: URL parsing

## 🚀 Usage

1. Clone the repository
2. Build the project:

```sh
cargo build
```

3. Run the crawler:

```sh
cargo run
```

4. The program will:
   - Start crawling from https://www.rust-lang.org
   - Print visited URLs and new links found
   - Generate a `graph.dot` file when finished

## 🎨 Visualization

To visualize the generated graph, install Graphviz and run:

```sh
dot -Tpng graph.dot > graph.png
```

## ⚠️ Note

The crawler is rate-limited to 1000 URLs to prevent excessive requests to web servers. Adjust the limit in the code if needed.

## 📝 License

MIT
