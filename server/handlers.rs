use actix_web::{AsyncResponder, Path, State, FutureResponse, HttpResponse};
use db::CreateTask;
use futures::Future;
use super::AppState;

pub fn new_task((name, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(CreateTask {
            title: name.into_inner(),
        }).from_err()
        .and_then(|res| match res {
            Ok(game) => Ok(HttpResponse::Ok().json(game)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }).responder()
}