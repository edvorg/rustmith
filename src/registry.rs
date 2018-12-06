use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;
use services::search::StubSearchService;
use services::render::RenderService;
use services::audio::AudioService;

pub struct Registry {
    pub console: ConsoleService,
    pub timeout: TimeoutService,
    pub search: StubSearchService,
    pub render: RenderService,
    pub audio: AudioService,
}