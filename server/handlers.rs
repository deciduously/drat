use super::AppState;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, Path, State};
use db::{CreateTask, GetTask};
use futures::Future;

pub fn get_task((id, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetTask {
            id: id.into_inner(),
        }).from_err()
        .and_then(|res| match res {
            Ok(task) => Ok(HttpResponse::Ok().json(task)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }).responder()
}

pub fn new_task((name, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(CreateTask {
            title: name.into_inner(),
        }).from_err()
        .and_then(|res| match res {
            Ok(task) => Ok(HttpResponse::Ok().json(task)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }).responder()
}
