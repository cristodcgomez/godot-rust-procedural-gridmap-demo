use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct GodotRustDemo;

mod player;
mod terrain;

use player::Player;
use terrain::Terrain;

// Function that registers all exposed classes to Godot
fn init(handle: InitHandle) {
    handle.add_class::<GodotRustDemo>();
    handle.add_class::<Player>();
    handle.add_class::<Terrain>();
}

impl GodotRustDemo {
    fn new(_owner: &Node) -> Self {
        GodotRustDemo
    }
}

// Only __one__ `impl` block can have the `#[methods]` attribute, which
// will generate code to automatically bind any exported methods to Godot.
#[methods]
impl GodotRustDemo {
    #[export]
    fn _ready(&self, _owner: &Node) {
        godot_print!("Hello, world!");
    }
}

godot_init!(init);