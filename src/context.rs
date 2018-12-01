use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;

pub struct Context {
    pub console: ConsoleService,
    pub timeout: TimeoutService,
}