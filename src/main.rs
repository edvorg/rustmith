#[macro_use]
extern crate stdweb;

#[macro_use]
extern crate yew;

mod context;
mod paging;
mod game;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;
use context::Context;
use std::time::Duration;

fn main() {
    yew::initialize();
    let console = ConsoleService::new();
    let timeout = TimeoutService::new();
    let context = Context { console, timeout };
    let app = App::<Context, paging::PagingModel>::new(context);
    app.mount_to_body();
    yew::run_loop();
}
