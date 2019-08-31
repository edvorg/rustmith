use failure::Error;
use rustmith_common::track::TrackData;
use yew::prelude::Callback;
use rustmith_common::track::TrackLoadResult;
use rustmith_common::track::TrackCreateResult;
use rustmith_common::track::SearchResponse;
use rustmith_common::track::Track;
use yew::services::fetch::FetchService;
use yew::services::fetch::FetchTask;
use yew::services::fetch::Request;
use yew::format::Json;
use yew::services::fetch::Response;
use yew::format::Nothing;

pub trait TrackService {
    fn create_track(&mut self, name: &str, youtube_id: &str, data: TrackData, callback: Callback<TrackCreateResult>) -> FetchTask;
    fn load_track(&mut self, track_id: &str, callback: Callback<TrackLoadResult>) -> FetchTask;
    fn search(&mut self, term: &str, continuation_token: Option<&String>, callback: Callback<SearchResponse>) -> FetchTask;
}

pub struct RemoteTrackService {
    http: FetchService,
}

//impl From<Track> for Result<String, Error> {
//    fn from(t: Track) -> Self {
//        Ok(serde_json::to_string(&t).unwrap())
//    }
//}

impl TrackService for RemoteTrackService {
    fn create_track(&mut self, name: &str, youtube_id: &str, data: TrackData, callback: Callback<TrackCreateResult>) -> FetchTask {
        let track = Track {
            id: "".to_string(),
            name: name.to_string(),
            youtube_id: youtube_id.to_string(),
            data,
        };
        let request = Request::post("http://localhost:8000/tracks")
            .header("Content-Type", "application/json")
            .body::<Json<&Track>>(Json(&track))
            .expect("Failed to build request.");
        self.http.fetch(request, Callback::from(move |response: Response<Json<Result<TrackCreateResult, Error>>>| {
            let (_meta, Json(body)) = response.into_parts();
            match body {
                Ok(r) => callback.emit(r),
                Err(_e) => callback.emit(TrackCreateResult::Error),
            }
        }))
    }

    fn load_track(&mut self, track_id: &str, callback: Callback<TrackLoadResult>) -> FetchTask {
        let request = Request::get(&format!("http://localhost:8000/tracks/{}", track_id))
            .header("Content-Type", "application/json")
            .body(Nothing)
            .expect("Failed to build request.");
        self.http.fetch(request, Callback::from(move |response: Response<Json<Result<TrackLoadResult, Error>>>| {
            let (_meta, Json(body)) = response.into_parts();
            match body {
                Ok(r) => callback.emit(r),
                Err(_e) => callback.emit(TrackLoadResult::Error),
            }
        }))
    }

    fn search(&mut self, term: &str, _continuation_token: Option<&String>, callback: Callback<SearchResponse>) -> FetchTask {
        let request = Request::get(&format!("http://localhost:8000/tracks?term={}", term))
            .header("Content-Type", "application/json")
            .body(Nothing)
            .expect("Failed to build request.");
        self.http.fetch(request, Callback::from(move |response: Response<Json<Result<SearchResponse, Error>>>| {
            let (_meta, Json(body)) = response.into_parts();
            match body {
                Ok(r) => callback.emit(r),
                Err(e) => {
                    js! {
                      console.log(@{format!("{:?}", e)});
                    }
                    callback.emit(SearchResponse::Error)
                },
            }
        }))
    }
}

impl Default for RemoteTrackService {
    fn default() -> Self {
        RemoteTrackService {
            http: FetchService::new(),
        }
    }
}

pub fn make_youtube_url(id: &str) -> String {
    format!("https://www.youtube.com/embed/{}?autoplay=1&loop=1", id)
}
