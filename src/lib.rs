
#[macro_use]
extern crate xplane_plugin;
use xplane_plugin::*;

extern crate xplm;
extern crate xpgl;
extern crate mapcore;

mod plugin;

xplane_plugin!(plugin::TerrainDisplayPlugin);
