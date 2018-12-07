use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;
use crate::services::search::StubSearchService;
use crate::services::render::RenderService;
use crate::services::audio::AudioService;

pub struct Registry {
    pub console: ConsoleService,
    pub timeout: TimeoutService,
    pub search: StubSearchService,
    pub render: RenderService,
    pub audio: AudioService,
}
