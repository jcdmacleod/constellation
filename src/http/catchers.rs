// Constellation
//
// Pluggable authoritative DNS server
// Copyright: 2018, Valerian Saliou <valerian@valeriansaliou.name>
// License: Mozilla Public License v2.0 (MPL v2.0)

use rocket_contrib::Json;

#[derive(Serialize)]
pub struct CatcherResponse {
    error: &'static str,
}

#[error(400)]
pub fn bad_request() -> Json<CatcherResponse> {
    Json(CatcherResponse { error: "bad_request" })
}

#[error(401)]
pub fn unauthorized() -> Json<CatcherResponse> {
    Json(CatcherResponse { error: "unauthorized" })
}

#[error(403)]
pub fn forbidden() -> Json<CatcherResponse> {
    Json(CatcherResponse { error: "forbidden" })
}

#[error(404)]
pub fn not_found() -> Json<CatcherResponse> {
    Json(CatcherResponse { error: "not_found" })
}

#[error(500)]
pub fn internal_server_error() -> Json<CatcherResponse> {
    Json(CatcherResponse { error: "internal_server_error" })
}
