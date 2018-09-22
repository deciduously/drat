use super::AppState;
use actix_web::{
    self, fs::NamedFile, AsyncResponder, FutureResponse, HttpRequest, HttpResponse, Path, State,
};
use db::{CreateTask, GetAllTasks, GetTask, ToggleTask};
use futures::{future::result, Future};
use std::{
    io::{BufReader, Read},
    path::PathBuf,
};

pub fn get_all_tasks(state: State<AppState>) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(GetAllTasks)
        .from_err()
        .and_then(|res| match res {
            Ok(task) => Ok(HttpResponse::Ok().json(task)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }).responder()
}

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

pub fn index(
    _req: &HttpRequest<AppState>,
) -> Box<Future<Item = HttpResponse, Error = actix_web::Error>> {
    let path: PathBuf = PathBuf::from("./public/index.html");

    let f = NamedFile::open(&path)
        .unwrap_or_else(|_| panic!("Could not open {}", path.to_str().unwrap()));
    let mut bfr = BufReader::new(f.file());
    let mut ret = String::new();
    bfr.read_to_string(&mut ret)
        .unwrap_or_else(|_| panic!("could not read index file"));

    result(Ok(HttpResponse::Ok().content_type("text/html").body(ret))).responder()
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

pub fn toggle_task((id, state): (Path<String>, State<AppState>)) -> FutureResponse<HttpResponse> {
    state
        .db
        .send(ToggleTask {
            id: id.into_inner(),
        }).from_err()
        .and_then(|res| match res {
            Ok(task) => Ok(HttpResponse::Ok().json(task)),
            Err(_) => Ok(HttpResponse::InternalServerError().into()),
        }).responder()
}
