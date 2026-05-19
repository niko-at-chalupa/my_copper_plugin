use endstone_copper::*;
use endstone_copper_macros::endstone_plugin;

#[derive(Default)]
pub struct ExamplePlugin;

#[endstone_plugin(
    name = "example_copper_plugin",
    version = "0.1.0",
    description = "Example Rust plugin for Endstone",
    author = "Niko"
)]
impl Plugin for ExamplePlugin {
    fn on_enable(&mut self) {
        println!("Hello, from Rust!")
    }
}
