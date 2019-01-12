#[macro_use]
extern crate yew;
#[macro_use]
extern crate stdweb;

mod registry;

mod model {
    pub mod editor;
    pub mod fps;
    pub mod game;
    pub mod guitar_effects;
    pub mod root;
    pub mod search;
    pub mod tuner;
}

mod services {
    pub mod ext;
    pub mod track;
    pub mod worker;
}

mod graphics {
    pub mod camera;
    pub mod objects;
    pub mod renderer;
    pub mod shaders;
}

mod view {
    pub mod editor;
    pub mod game;
    pub mod fps;
    pub mod guitar_effects;
    pub mod root;
    pub mod search;
    pub mod tuner;
}

use crate::registry::Registry;
use crate::services::track::RemoteTrackService;
use stdweb::web::document;
use stdweb::web::INonElementParentNode;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;
use yew_audio::AudioService;

fn main() {
    yew::initialize();
    let console = ConsoleService::new();
    let timeout = TimeoutService::new();
    let audio = AudioService::default();
    let track = RemoteTrackService::default();
    let registry = Registry {
        console,
        timeout,
        audio,
        track,
    };
    let app = App::<Registry, model::root::RootModel>::new(registry);
    app.mount(document().get_element_by_id("app").unwrap());
    yew::run_loop();
}
