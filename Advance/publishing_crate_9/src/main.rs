// main.rs
// This is the entry point for a binary executable that uses your library crate
// You need to declare the dependency on your crate with the extern keyword
extern crate publishing_crate_9;

// You can use the functions from your crate with the use keyword
use publishing_crate_9::publish::greet;

fn main() {
    // Call the greet function from your crate
    greet();
}
