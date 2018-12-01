use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;

pub struct Registry {
    pub console: ConsoleService,
    pub timeout: TimeoutService,
}