use crate::registry::Registry;
use yew::prelude::Component;
use yew::prelude::Env;
use yew::prelude::Html;
use yew::prelude::Renderable;
use yew_audio::AudioNode;
use yew_audio::Destination;
use yew_audio::MediaStreamSource;

pub struct GuitarEffectsModel {
    destination: Destination,
    mic: Option<MediaStreamSource>,
}

pub enum GuitarEffectsMessage {}

#[derive(Clone, PartialEq)]
pub struct GuitarEffectsProps {
    pub mic: Option<MediaStreamSource>,
}

impl Default for GuitarEffectsProps {
    fn default() -> Self {
        GuitarEffectsProps { mic: None }
    }
}

impl Component<Registry> for GuitarEffectsModel {
    type Message = GuitarEffectsMessage;
    type Properties = GuitarEffectsProps;

    fn create(props: Self::Properties, env: &mut Env<Registry, Self>) -> Self {
        GuitarEffectsModel {
            destination: env.audio.destination(),
            mic: props.mic,
        }
    }

    fn update(&mut self, _msg: Self::Message, _env: &mut Env<Registry, Self>) -> bool {
        false
    }

    fn change(&mut self, props: Self::Properties, _env: &mut Env<Registry, Self>) -> bool {
        match props.mic {
            Some(mic) => {
                mic.connect(&self.destination);
                self.mic = Some(mic);
                true
            }
            None => false,
        }
    }
}

impl Renderable<Registry, GuitarEffectsModel> for GuitarEffectsModel {
    fn view(&self) -> Html<Registry, GuitarEffectsModel> {
        html! {
            <div class="game-effects",>
              <div>
                { "Overdrive" }
              </div>
              <div>
                { "Distoration" }
              </div>
              <div>
                { "Compressor" }
              </div>
            </div>
        }
    }
}
