#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate yew;
#[macro_use]
extern crate serde_derive;
extern crate stdweb_derive;
extern crate webgl_rendering_context;
extern crate yew_audio;

pub mod common;
mod fps;
mod game;
mod registry;
mod root;
mod search;

mod services {
    pub mod ext;
    pub mod search;
    pub mod worker;
}

mod graphics {
    pub mod algebra;
    pub mod renderer;
    pub mod shaders;
}

use crate::registry::Registry;
use crate::services::search::StubSearchService;
use stdweb::web::document;
use stdweb::web::INonElementParentNode;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;
use yew_audio::AudioService;

#[test]
fn test_fail() {
    assert_eq!(1, 2);
}

fn main() {
    yew::initialize();
    let console = ConsoleService::new();
    let timeout = TimeoutService::new();
    let search = StubSearchService::new();
    let audio = AudioService::new();
    let registry = Registry {
        console,
        timeout,
        search,
        audio,
    };
    let app = App::<Registry, root::RootModel>::new(registry);
    app.mount(document().get_element_by_id("app").unwrap());
    yew::run_loop();
}
