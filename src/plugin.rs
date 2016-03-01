use xplane_plugin::*;
use xpgl::gl;
use xplm;

use xplm::ui::*;
use xplm::graphics::*;
use xplm::graphics::window::*;

use mapcore::map::Map;
use mapcore::layer::Layer;
use mapcore::{Latitude, Longitude, LatLon, LatLonRect, Polygon};
use mapcore::projection::Projection;
use mapcore::equirectangular::EquirectangularProjection;

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
        let mut map = Map::new(EquirectangularProjection);
        map.add_layer(WorldLayer::new());
        map.add_layer(TestLayer);
        {
            let local_draw_window = move |window: &mut Window| {

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
                map.draw(rect.left, rect.bottom, (rect.right - rect.left), (rect.top - rect.bottom));
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

struct TestLayer;

impl Layer for TestLayer {
    fn draw(&self, projection: &Projection, x: i32, y: i32, width: i32, height: i32) {
        set_state(&GRAPHICS_STATE_2D);
        // Draw a projected latitude-longitude rectangle
        let poly = Polygon::new(&[LatLon{ latitude: Latitude(37.41), longitude: Longitude(-122.29) },
                                LatLon{ latitude: Latitude(47.66), longitude: Longitude(-122.27) },
                                LatLon{ latitude: Latitude(51.507222), longitude: Longitude(-0.1275) },
                                LatLon{ latitude: Latitude(40.383333), longitude: Longitude(-3.716667) }]);
        let projected = projection.project_poly(&poly);
        unsafe {
            gl::Color3f(0.0, 0.0, 1.0);
            gl::Begin(gl::QUADS);
            for point in projected.points() {
                gl::Vertex2d((x as f64) + point.x, (y as f64) + point.y);
            }
            gl::End();
        }
    }

    fn bounds(&self) -> Option<LatLonRect> {
        None
    }
}

///
/// Draws a rectangle covering the whole world
///
struct WorldLayer {
    /// A Lat/Lon polygon that spans the world
    world: Polygon<LatLon>
}

impl WorldLayer {
    pub fn new() -> WorldLayer {
        WorldLayer {
            world: Polygon::new(&[LatLon{ latitude: Latitude(-90.0), longitude: Longitude(-180.0) },
                                    LatLon{ latitude: Latitude(90.0), longitude: Longitude(-180.0) },
                                    LatLon{ latitude: Latitude(90.0), longitude: Longitude(180.0) },
                                    LatLon{ latitude: Latitude(-90.0), longitude: Longitude(180.0) }]),
        }
    }
}

impl Layer for WorldLayer {
    fn draw(&self, projection: &Projection, x: i32, y: i32, width: i32, height: i32) {
        set_state(&GRAPHICS_STATE_2D);
        let projected = projection.project_poly(&self.world);
        unsafe {
            gl::Color3f(0.0, 0.5, 1.0);
            gl::Begin(gl::QUADS);
            for point in projected.points() {
                gl::Vertex2d((x as f64) + point.x, (y as f64) + point.y);
            }
            gl::End();
        }
    }

    fn bounds(&self) -> Option<LatLonRect> {
        None
    }
}
