#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate redis;
extern crate uuid;

use rocket_contrib::json::Json;
use rocket_contrib::serve::StaticFiles;
use rustmith_common::track::*;
use rocket::response::Responder;
use rocket::Request;
use rocket::Response;
use rocket::http::Status;
use redis::RedisError;
use uuid::Uuid;
use redis::Commands;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum ApiError {
    DatabaseError,
    InvalidFormatError,
}

impl<'a> Responder<'a> for ApiError {
    fn respond_to(self, _request: &Request) -> Result<Response<'a>, Status> {
        Result::Err(Status {
            code: 501,
            reason: "Api error"
        })
    }
}

impl From<RedisError> for ApiError {
    fn from(_: RedisError) -> Self {
        ApiError::DatabaseError
    }
}

impl From<ParseIntError> for ApiError {
    fn from(_: ParseIntError) -> Self {
        ApiError::InvalidFormatError
    }
}

impl From<serde_json::error::Error> for ApiError {
    fn from(_: serde_json::error::Error) -> Self {
        ApiError::InvalidFormatError
    }
}

#[post("/tracks", data = "<track>")]
fn post_track(mut track: Json<Track>) -> Result<Json<TrackCreateResult>, ApiError> {
    let id = Uuid::new_v4();
    track.id = id.to_string();
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    let serialized = serde_json::to_string(&track.0)?;
    let _: () = con.hset("tracks", &id.to_string(), serialized)?;
    Result::Ok(Json(TrackCreateResult::Created(id.to_string(), track.0)))
}

#[get("/tracks?<term>")]
fn search_track(term: String) -> Result<Json<SearchResponse>, ApiError> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    let mut items: Vec<SearchItem> = Vec::new();
    let results: redis::Iter<(String, String)> = con.hscan("tracks")?;
    for (id, result) in results {
        let track: Track = serde_json::from_str(&result)?;
        if track.name.contains(&term) {
            items.push(SearchItem {
                name: track.name,
                id,
                youtube_id: track.youtube_id,
            })
        }
    }
    Result::Ok(Json(SearchResponse::Result {
        term,
        items,
        continuation_token: None
    }))
}

#[get("/tracks/<id>")]
fn get_track(id: String) -> Result<Json<TrackLoadResult>, ApiError> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let con = client.get_connection()?;
    let result: String = con.hget("tracks", &id.to_string())?;
    let track: Track = serde_json::from_str(&result)?;
    Result::Ok(Json(TrackLoadResult::Loaded(track.data)))
}

fn main() {
    let static_files = StaticFiles::from("./target/deploy");
    let routes = routes![post_track, search_track, get_track];
    rocket::ignite()
        .mount("/", routes)
        .mount("/", static_files).launch();
}
