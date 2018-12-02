use stdweb::Value;
use yew::prelude::*;
use yew::services::Task;
use stdweb::Number;
use stdweb::unstable::TryInto;

/// A handle to cancel a render task.
pub struct RenderTask(Option<Value>);

/// A service to request animation frames.
#[derive(Default)]
pub struct RenderService {}

impl RenderService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn request_animation_frame(&mut self, callback: Callback<f64>) -> RenderTask {
        let callback = move |v| {
            let time: f64 = match v {
                Value::Number(n) => n.try_into().unwrap(),
                _ => 0.0
            };
            callback.emit(time);
        };
        let handle = js! {
            var callback = @{callback};
            var action = function(time) {
                callback(time);
                callback.drop();
            };
            return {
                render_id: requestAnimationFrame(action),
                callback: callback,
            };
        };
        RenderTask(Some(handle))
    }
}

impl Task for RenderTask {
    fn is_active(&self) -> bool {
        self.0.is_some()
    }
    fn cancel(&mut self) {
        let handle = self.0.take().expect("tried to cancel render twice");
        js! { @(no_return)
            var handle = @{handle};
            cancelAnimationFrame(handle.timeout_id);
            handle.callback.drop();
        }
    }
}

impl Drop for RenderTask {
    fn drop(&mut self) {
        if self.is_active() {
            self.cancel();
        }
    }
}
