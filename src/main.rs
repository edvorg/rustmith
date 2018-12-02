#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb_derive;
#[macro_use]
extern crate webgl_rendering_context;

mod registry;
mod root;
mod search;
mod game;

mod services {
    pub mod render;
}

mod graphics {
    pub mod algebra;
    pub mod shaders;
    pub mod renderer;
}

use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;
use registry::Registry;
use services::render::RenderService;

fn main() {
    yew::initialize();
    let console = ConsoleService::new();
    let timeout = TimeoutService::new();
    let render = RenderService::new();
    let registry = Registry { console, timeout, render };
    let app = App::<Registry, root::RootModel>::new(registry);
    app.mount_to_body();
    yew::run_loop();
}
