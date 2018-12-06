use stdweb::Value;

pub trait Node {
    fn js(&self) -> &Value;
    fn connect(&self, to: &Node);
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

pub struct AudioService {
    context: Value,
}

impl Node for Oscillator {
    fn js(&self) -> &Value {
        &self.js
    }

    fn connect(&self, to: &Node) {
        js! { @{&self.js}.connect(@{to.js()}); }
    }
}

impl Node for Gain {
    fn js(&self) -> &Value {
        &self.js
    }

    fn connect(&self, to: &Node) {
        js! { @{&self.js}.connect(@{to.js()}); }
    }
}

impl Node for Destination {
    fn js(&self) -> &Value {
        &self.js
    }

    fn connect(&self, to: &Node) {
        js! { @{&self.js}.connect(@{to.js()}); }
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

}