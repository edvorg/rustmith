use crate::graphics::renderer::RendererModel;
use crate::guitar_effects::GuitarEffectsModel;
use crate::registry::Registry;
use crate::services::ext::WindowExt;
use crate::services::track::TrackLoadResult;
use crate::services::track::TrackService;
use crate::tuner::TunerModel;
use rustmith_common::track::Track;
use stdweb::web::window;
use yew::prelude::*;
use yew_audio::MediaStream;
use yew_audio::MediaStreamSource;

/// this type of message is used for inter-component communication
pub enum RoutingMessage {
    /// switch to search screen
    ExitGame,
}

pub enum GameMessage {
    Route(RoutingMessage),
    ConnectMicrophone(MediaStream),
    TrackReceived(TrackLoadResult),
}

struct GameStats {
    notes_missed: u16,
    notes_hit: u16,
    mastery: u16,
}

pub struct GameModel {
    on_signal: Option<Callback<RoutingMessage>>,
    #[allow(dead_code)]
    song_id: Option<String>,
    song_url: Option<String>,
    track: Option<Track>,
    stats: GameStats,
    mic: Option<MediaStreamSource>,
}

#[derive(PartialEq, Clone)]
pub struct GameProps {
    pub onsignal: Option<Callback<RoutingMessage>>,
    pub songid: Option<String>,
    pub songurl: Option<String>,
}

impl Default for GameProps {
    fn default() -> Self {
        GameProps {
            onsignal: None,
            songid: None,
            songurl: None,
        }
    }
}

impl Component<Registry> for GameModel {
    type Message = GameMessage;
    type Properties = GameProps;

    fn create(props: Self::Properties, env: &mut Env<Registry, Self>) -> Self {
        env.console.log("creating game model");
        GameModel::fetch_mic(env);
        if let Some(song_id) = &props.songid {
            GameModel::fetch_track(env, song_id);
        }
        GameModel {
            on_signal: props.onsignal,
            song_id: props.songid,
            song_url: props.songurl,
            track: None,
            stats: GameStats {
                notes_missed: 0,
                notes_hit: 0,
                mastery: 0,
            },
            mic: None,
        }
    }

    fn update(&mut self, msg: Self::Message, env: &mut Env<Registry, Self>) -> bool {
        match msg {
            GameMessage::Route(message) => {
                if let Some(callback) = &self.on_signal {
                    callback.emit(message);
                } else {
                    env.console.warn("Something is wrong, router not found");
                }
                false
            }
            GameMessage::ConnectMicrophone(mic) => {
                env.console.log("Established mic connection");
                let mic = env.audio.create_media_stream_source(mic);
                window().set_source(&mic);
                self.mic = Some(mic);
                true
            }
            GameMessage::TrackReceived(TrackLoadResult::Loaded(track)) => {
                self.track = Some(track);
                true
            }
            GameMessage::TrackReceived(TrackLoadResult::Error) => {
                self.track = None;
                env.console.warn(&format!("Unable to load track {:?}", &self.song_id));
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties, env: &mut Env<Registry, Self>) -> bool {
        if let Some(song_id) = &props.songid {
            GameModel::fetch_track(env, song_id);
        }
        false
    }
}

impl Renderable<Registry, GameModel> for GameModel {
    fn view(&self) -> Html<Registry, GameModel> {
        html! {
          <div class="game",>
            <div class="game-view",>
              <button id="exit-button", onclick = |_| GameMessage::Route(RoutingMessage::ExitGame),> { "exit" } </button>
              <RendererModel: track=&self.track, />
            </div>
            <div class="game-video",>
              <iframe id="video-clip",
                      src=&self.song_url.clone().unwrap(),
                      frameborder="0",
                      allow="accelerometer; autoplay; encrypted-media; gyroscope; picture-in-picture",>
              </iframe>
            </div>
            <div class="game-stats",>
              <div>
                { format!("Notes missed {}", &self.stats.notes_missed) }
              </div>
              <div>
                { format!("Notes hit {}", &self.stats.notes_hit) }
              </div>
              <div>
                { format!("Mastery {}%", &self.stats.mastery) }
              </div>
            </div>
            <GuitarEffectsModel: mic=self.mic.clone(), />
            <TunerModel: mic=self.mic.clone(), />
          </div>
        }
    }
}

impl GameModel {
    fn fetch_track(env: &mut Env<Registry, GameModel>, song_id: &str) {
        let on_song = env.send_back(GameMessage::TrackReceived);
        env.track.load_track(song_id, on_song);
    }
}

impl GameModel {
    fn fetch_mic(env: &mut Env<Registry, GameModel>) {
        let on_mic = env.send_back(GameMessage::ConnectMicrophone);
        env.audio.get_user_media().call_audio(on_mic);
    }
}
