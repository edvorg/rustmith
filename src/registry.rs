use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;
use services::render::RenderService;

pub struct Registry {
    pub console: ConsoleService,
    pub timeout: TimeoutService,
    pub render: RenderService,
}