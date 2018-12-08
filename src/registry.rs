use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;
use crate::services::search::StubSearchService;
use yew_audio::AudioService;

pub struct Registry {
    pub console: ConsoleService,
    pub timeout: TimeoutService,
    pub search: StubSearchService,
    pub audio: AudioService,
}
