use stdweb::Value;
use yew::prelude::*;
use stdweb::unstable::TryInto;

pub trait AudioNode {
    fn js(&self) -> &Value;

    fn connect(&self, to: &AudioNode) {
        js! { @{&self.js()}.connect(@{to.js()}); }
    }

    fn disconnect(&self) {
        js! { @{&self.js()}.disconnect(); }
    }
}

pub struct Oscillator {
    js: Value,
}

pub struct Gain {
    js: Value,
}

pub struct Destination {
    js: Value,
}

pub struct MediaStreamSource {
    js: Value,
}

pub struct ScriptProcessor {
    js: Value,
}

pub struct GetUserMedia {
    js: Value,
}

pub struct MediaStream {
    js: Value,
}

pub struct AudioService {
    context: Value,
}

pub struct AudioProcessingEvent {
    js: Value,
}

pub struct InputBuffer {
    js: Value,
}

impl AudioNode for Oscillator {
    fn js(&self) -> &Value {
        &self.js
    }
}

impl AudioNode for Gain {
    fn js(&self) -> &Value {
        &self.js
    }
}

impl AudioNode for Destination {
    fn js(&self) -> &Value {
        &self.js
    }
}

impl AudioNode for MediaStreamSource {
    fn js(&self) -> &Value {
        &self.js
    }
}

impl AudioNode for ScriptProcessor {
    fn js(&self) -> &Value {
        &self.js
    }
}

impl GetUserMedia {
    pub fn call_audio(&self, callback: Callback<MediaStream>) {
        let callback = move |v| {
            callback.emit(
                MediaStream {
                    js: v,
                }
            )
        };
        js! {
            @{&self.js}.call(navigator, { "audio": true }, @{callback}, function() {});
        }
    }
}

impl Oscillator {
    pub fn set_frequency(&self, value: f32) {
        js! { @{&self.js}.frequency.value = @{value}; }
    }

    pub fn start(&self) {
        js! { @{&self.js}.start(); }
    }
}

impl AudioProcessingEvent {
    pub fn input_buffer(&self) -> InputBuffer {
        InputBuffer {
            js: js! { return @{&self.js}.inputBuffer; },
        }
    }
}

impl InputBuffer {
    pub fn get_channel_data_buffer(&self, channel: u8) -> Vec<f64> {
        js! (
            return Array.prototype.slice.call(@{&self.js}.getChannelData(@{channel}));
        ).try_into().unwrap()
    }
}

impl ScriptProcessor {
    pub fn set_onaudioprocess(&self, callback: Callback<AudioProcessingEvent>) {
        let callback = move |v| {
            callback.emit(
                AudioProcessingEvent {
                    js: v,
                }
            );
        };
        js! {
            var callback = @{callback};
            @{&self.js}.onaudioprocess = callback;
        }
    }
}

impl Gain {
    pub fn set_value(&self, value: f32) {
        js! { @{&self.js}.gain.value = @{value}; }
    }
}

impl AudioService {
    pub fn new() -> AudioService {
        AudioService {
            context: js! { return new AudioContext(); }
        }
    }

    pub fn create_oscillator(&self) -> Oscillator {
        Oscillator {
            js: js! { return @{&self.context}.createOscillator(); },
        }
    }

    pub fn create_gain(&self) -> Gain {
        Gain {
            js: js! { return @{&self.context}.createGain(); },
        }
    }

    pub fn destination(&self) -> Destination {
        Destination {
            js: js! { return @{&self.context}.destination; },
        }
    }

    pub fn sample_rate(&self) -> f64 {
        js! (
          return @{&self.context}.sampleRate;
        ).try_into().unwrap()
    }

    pub fn create_script_processor(&self, buffer_size: i32, input_channels: i32, output_channels: i32) -> ScriptProcessor {
        ScriptProcessor {
            js: js! { return @{&self.context}.createScriptProcessor(@{buffer_size}, @{input_channels}, @{output_channels}); },
        }
    }

    pub fn get_user_media(&self) -> GetUserMedia {
        GetUserMedia {
            js: js! {
                var get_user_media = navigator.getUserMedia;
                get_user_media = get_user_media || navigator.webkitGetUserMedia;
                get_user_media = get_user_media || navigator.mozGetUserMedia;
                return get_user_media
            }
        }
    }

    pub fn create_media_stream_source(&self, stream: MediaStream) -> MediaStreamSource {
        MediaStreamSource {
            js: js! { return @{&self.context}.createMediaStreamSource(@{&stream.js}); },
        }
    }
}
