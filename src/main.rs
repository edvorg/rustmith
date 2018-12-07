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

pub mod registry;
pub mod root;
pub mod search;
pub mod game;
pub mod fps;
pub mod common;

mod services {
    pub mod search;
    pub mod render;
    pub mod audio;
    pub mod ext;
    pub mod worker;
}

mod graphics {
    pub mod algebra;
    pub mod shaders;
    pub mod renderer;
}

use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;
use crate::registry::Registry;
use crate::services::search::StubSearchService;
use crate::services::render::RenderService;
use crate::services::audio::AudioService;
use stdweb::web::INonElementParentNode;
use stdweb::web::document;

fn main() {
    yew::initialize();
    let console = ConsoleService::new();
    let timeout = TimeoutService::new();
    let search = StubSearchService::new();
    let render = RenderService::new();
    let audio = AudioService::new();
    let registry = Registry { console, timeout, search, render, audio };
    let app = App::<Registry, root::RootModel>::new(registry);
    app.mount(document().get_element_by_id("app").unwrap());
    yew::run_loop();
}
