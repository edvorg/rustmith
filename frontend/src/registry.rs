use crate::services::track::RemoteTrackService;
use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;
use yew_audio::AudioService;

pub struct Registry {
    pub console: ConsoleService,
    pub timeout: TimeoutService,
    pub audio: AudioService,
    pub track: RemoteTrackService,
}
