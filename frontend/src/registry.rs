use crate::services::search::StubSearchService;
use crate::services::track::StubTrackService;
use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;
use yew_audio::AudioService;

pub struct Registry {
    pub console: ConsoleService,
    pub timeout: TimeoutService,
    pub search: StubSearchService,
    pub audio: AudioService,
    pub track: StubTrackService,
}
