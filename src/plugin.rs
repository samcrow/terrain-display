use xplane_plugin::*;
use gl;
use xplm;

use xplm::ui::Rect;
use xplm::graphics::*;
use xplm::graphics::window::*;


use std::mem;

fn draw_window() {
    println!("Drawing");
    gl::Vertex2f(100, 600);
}

pub struct TerrainDisplayPlugin {
    /// The window that displays graphics
    window: Option<Window>,
}

impl Plugin for TerrainDisplayPlugin {
    fn start() -> Option<Self> {
        // Load OpenGL symbols
        gl::load_with(|name| {
            unsafe { mem::transmute(xplm::find_symbol(name)) }
        });

        Some(TerrainDisplayPlugin {
            window: None,
        })
    }
    fn enable(&mut self) {
        self.window = Some(Window::new(&Rect { left: 100, top: 1000, right: 400, bottom: 600 }));
        self.window.as_mut().unwrap().set_draw_callback(draw_window);
        self.window.as_mut().unwrap().set_visible(true);
    }
    fn disable(&mut self) {
        self.window = None;
    }

    fn info<'a, 'b, 'c>(&self) -> PluginInfo<'a, 'b, 'c> {
        PluginInfo {
            name: "Terrain Display",
            signature: "org.samcrow.dev.terrain_display",
            description: "Draws a display of the terrain around the aircraft",
        }
    }

    fn stop(&mut self) {

    }
}
