#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;

mod fps;
mod game;
mod guitar_effects;
mod registry;
mod root;
mod search;
mod tuner;

mod services {
    pub mod ext;
    pub mod search;
    pub mod track;
    pub mod worker;
}

mod graphics {
    pub mod camera;
    pub mod objects;
    pub mod renderer;
    pub mod shaders;
}

use crate::registry::Registry;
use crate::services::search::StubSearchService;
use crate::services::track::StubTrackService;
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
    let search = StubSearchService::default();
    let audio = AudioService::default();
    let track = StubTrackService::default();
    let registry = Registry {
        console,
        timeout,
        search,
        audio,
        track,
    };
    let app = App::<Registry, root::RootModel>::new(registry);
    app.mount(document().get_element_by_id("app").unwrap());
    yew::run_loop();
}
