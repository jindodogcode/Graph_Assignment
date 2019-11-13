# Rust/WASM Pathfinding Webpage

## About

This is an extra credit project I made for a class. The project was to
create a graph with nodes representing locations in the US and implement
a few pathing algorithms. The extra credit was to visualize the graph
and the algorithms.

Before working on this assignment, I had never used the canvas element,
Javascript, or WebAssembly. So this assignment represents the efforts of
a novice learning new technologies.

The project is divided into 3 parts. It started as an implementation of
a graph and related algorithms, written in Rust, which can be found in
the graph-lib directory. The actual web code is divided into the
web-bind directory, which contains the Rust code to be compiled to
WebAssembly, and the WWW directory, which contains the HTML, CS, and
Javascript.

## Example

[![Video example](http://img.youtube.com/vi/151Tjz-tloU/0.jpg)](http://www.youtube.com/watch?v=151Tjz-tloU)

## Build

- The rust compiler and tools: https://www.rust-lang.org/tools/install
- Node and NPM: https://www.npmjs.com/get-npm
- WebAssembly target for Rust: terminal: rustup target add wasm32-unknown-unknown
- wasm-pack(https://github.com/rustwasm/wasm-pack) 
- Go to the www directory and run: npm install
- npm run build

## Run

- cd www
- npm run serve
