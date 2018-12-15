use stdweb::Value;
use yew::prelude::Callback;

pub struct Worker {
    js: Value,
}

impl Worker {
    pub fn new(path: &str) -> Worker {
        Worker {
            js: js! { return new Worker(@{path}); },
        }
    }

    pub fn add_event_listener(&self, message: &str, callback: Callback<Value>) {
        let callback = move |v| callback.emit(v);
        js! {
            @{&self.js}.addEventListener(@{message}, @{callback});
        }
    }

    pub fn post_message(&self, message: Value) {
        js! {
            @{&self.js}.postMessage(@{message});
        }
    }
}
