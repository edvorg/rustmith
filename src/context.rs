use yew::services::console::ConsoleService;
use yew::services::timeout::TimeoutService;
use yew::services::interval::IntervalService;

pub struct Context {
    pub console: ConsoleService,
    pub timeout: TimeoutService,
}