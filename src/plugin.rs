use xplane_plugin::*;
use xpgl::gl;
use xplm;

use xplm::ui::*;
use xplm::graphics::*;
use xplm::graphics::window::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

/// The graphics state used for 2D drawing
const GRAPHICS_STATE_2D : GraphicsState = GraphicsState {
    fog: false,
    lighting: false,
    alpha_testing: false,
    alpha_blending: true,
    depth_testing: false,
    depth_writing: false,
    textures: 0,
};

pub struct TerrainDisplayPlugin {
    /// The window that displays graphics
    window: Option<Rc<RefCell<Window>>>,
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
        {
            let local_draw_window = |window: &mut Window| {

                let rect = window.get_geometry();

                set_state(&GRAPHICS_STATE_2D);
                unsafe {
                    gl::Color3f(0.0, 0.5, 0.0);
                    gl::Begin(gl::QUADS);
                    gl::Vertex2i(rect.left, rect.bottom);
                    gl::Vertex2i(rect.left, rect.top);
                    gl::Vertex2i(rect.right, rect.top);
                    gl::Vertex2i(rect.right, rect.bottom);
                    gl::End();
                }
            };

            let mut window = self.window.as_mut().unwrap().borrow_mut();
            window.set_draw_callback(local_draw_window);
            window.set_visible(true);
        }
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
