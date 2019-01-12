use crate::registry::Registry;
use crate::services::track::make_youtube_url;
use stdweb::unstable::TryInto;
use yew::prelude::*;
use rustmith_common::track::TrackCreateResult;
use crate::services::track::TrackService;
use rustmith_common::track::TrackData;
use yew::services::fetch::FetchTask;

fn now() -> f64 {
    js! (
      return performance.now();
    )
    .try_into()
    .unwrap()
}

pub enum RoutingMessage {
    Exit,
    ExitAndShowTrack(String),
}

pub enum EditorMessage {
    Route(RoutingMessage),
    UpdateSongName(String),
    UpdateSongYoutubeId(String),
    UpdateSongContent(String),
    SaveAndExit,
    Error(&'static str),
    SetPosition { string: u8, fret: u8 },
    LogFret,
    StartRecording,
}

pub struct EditorModel {
    pub onsignal: Option<Callback<RoutingMessage>>,
    pub song_name: Option<String>,
    pub song_youtube_id: Option<String>,
    pub song_content: Option<String>,
    pub current_string: u8,
    pub current_fret: u8,
    pub recording: bool,
    pub recording_from: u64,
    pub song_url: Option<String>,
    pub task: Option<FetchTask>,
}

#[derive(Clone, PartialEq)]
pub struct EditorProps {
    pub onsignal: Option<Callback<RoutingMessage>>,
}

impl Default for EditorProps {
    fn default() -> Self {
        EditorProps { onsignal: None }
    }
}

impl Component<Registry> for EditorModel {
    type Message = EditorMessage;
    type Properties = EditorProps;

    fn create(props: <Self as Component<Registry>>::Properties, _context: &mut Env<Registry, Self>) -> Self {
        EditorModel {
            onsignal: props.onsignal,
            song_name: None,
            song_youtube_id: None,
            song_content: None,
            current_string: 0,
            current_fret: 0,
            recording: false,
            recording_from: 0,
            song_url: None,
            task: None,
        }
    }

    fn update(&mut self, msg: <Self as Component<Registry>>::Message, context: &mut Env<Registry, Self>) -> bool {
        match msg {
            EditorMessage::StartRecording => {
                self.recording_from = now() as u64;
                self.recording = true;
                true
            }
            EditorMessage::SetPosition { string, fret } => {
                self.current_string = string;
                self.current_fret = fret;
                false
            }
            EditorMessage::LogFret => {
                let starts_at = now() as u64 - self.recording_from;
                let ends_at = starts_at + 100;
                let line = format!("fret:{}:{}:{}:{}", starts_at, ends_at, self.current_fret + 1, self.current_string + 1);
                if let Some(content) = &self.song_content {
                    self.song_content = Some(format!("{}\n{}", content, line));
                } else {
                    self.song_content = Some(line);
                }
                true
            }
            EditorMessage::Error(e) => {
                self.task = None;
                context.console.error(&format!("error {}", e));
                false
            }
            EditorMessage::Route(m) => {
                self.task = None;
                context.console.log(&format!("{:?} {:?}", &self.song_youtube_id, &self.song_name));
                if let Some(signal) = &self.onsignal {
                    signal.emit(m);
                }
                true
            }
            EditorMessage::SaveAndExit => match (&self.song_content, &self.song_name, &self.song_youtube_id) {
                (Some(content), Some(name), Some(youtube_id)) => {
                    let callback = context.send_back(|r: TrackCreateResult| match r {
                        TrackCreateResult::Created(id, _) => EditorMessage::Route(RoutingMessage::ExitAndShowTrack(id)),
                        TrackCreateResult::Error => EditorMessage::Error("create error"),
                    });
                    if let Ok(data) = TrackData::parse(content) {
                        self.task = Some(context.track.create_track(name, youtube_id, data, callback));
                    }
                    true
                }
                _ => {
                    context.console.error("Not all fields are set");
                    false
                }
            },
            EditorMessage::UpdateSongName(n) => {
                self.song_name = Some(n);
                true
            }
            EditorMessage::UpdateSongYoutubeId(n) => {
                self.song_url = Some(make_youtube_url(&n));
                self.song_youtube_id = Some(n);
                true
            }
            EditorMessage::UpdateSongContent(n) => {
                if n.is_empty() {
                    self.song_content = None;
                } else {
                    self.song_content = Some(n);
                }
                true
            }
        }
    }

    fn change(&mut self, props: <Self as Component<Registry>>::Properties, _context: &mut Env<Registry, Self>) -> bool {
        self.onsignal = props.onsignal;
        true
    }
}
