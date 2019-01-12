use crate::registry::Registry;
use crate::services::ext::WindowExt;
use crate::services::track::TrackService;
use rustmith_common::track::TrackData;
use stdweb::web::window;
use yew::prelude::*;
use yew_audio::MediaStream;
use yew_audio::MediaStreamSource;
use rustmith_common::track::TrackLoadResult;
use yew::services::fetch::FetchTask;

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

pub struct GameStats {
    pub notes_missed: u16,
    pub notes_hit: u16,
    pub mastery: u16,
}

pub struct GameModel {
    on_signal: Option<Callback<RoutingMessage>>,
    #[allow(dead_code)]
    song_id: Option<String>,
    pub song_url: Option<String>,
    pub track: Option<TrackData>,
    pub stats: GameStats,
    pub mic: Option<MediaStreamSource>,
    task: Option<FetchTask>,
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
        let mut task: Option<FetchTask> = None;
        if let Some(song_id) = &props.songid {
            task = Some(GameModel::fetch_track(env, song_id));
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
            task,
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
                self.task = None;
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
            self.task = Some(GameModel::fetch_track(env, song_id));
        }
        false
    }
}

impl GameModel {
    fn fetch_track(env: &mut Env<Registry, GameModel>, song_id: &str) -> FetchTask {
        let on_song = env.send_back(GameMessage::TrackReceived);
        env.track.load_track(song_id, on_song)
    }
}

impl GameModel {
    fn fetch_mic(env: &mut Env<Registry, GameModel>) {
        let on_mic = env.send_back(GameMessage::ConnectMicrophone);
        env.audio.get_user_media().call_audio(on_mic);
    }
}
