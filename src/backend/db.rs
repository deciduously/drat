//! Db executor actor
use actix::prelude::*;
use actix_web::{error, Error, Result};
use diesel::{
    self,
    prelude::*,
    r2d2::{ConnectionManager, Pool},
};
use dotenv::dotenv;
use std::env::var;
use uuid;

use models;
use schema;

pub fn establish_connection_manager() -> ConnectionManager<PgConnection> {
    dotenv().ok();

    let database_url = var("DATABASE_URL").expect("DATABASE_URL must be set");

    ConnectionManager::new(database_url)
}

/// This is db executor actor. We are going to run 3 of them in parallel.
pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

/// Message to create a new task
pub struct CreateTask {
    pub title: String,
}

impl Message for CreateTask {
    type Result = Result<models::Task, Error>;
}

/// Message to retrieve a task by id
pub struct GetTask {
    pub id: String,
}

impl Message for GetTask {
    type Result = Result<models::Task, Error>;
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

impl Handler<CreateTask> for DbExecutor {
    type Result = Result<models::Task, Error>;

    fn handle(&mut self, msg: CreateTask, _: &mut Self::Context) -> Self::Result {
        use self::schema::tasks::dsl::*;

        let uuid = format!("{}", uuid::Uuid::new_v4());
        let new_task = models::NewTask {
            id: &uuid,
            title: &msg.title,
            completed: false,
        };

        let conn: &PgConnection = &self.0.get().unwrap();

        diesel::insert_into(tasks)
            .values(&new_task)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting task"))?;

        let mut ts = tasks
            .filter(id.eq(&uuid))
            .load::<models::Task>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading new task"))?;

        Ok(ts.pop().unwrap())
    }
}

impl Handler<GetTask> for DbExecutor {
    type Result = Result<models::Task, Error>;

    fn handle(&mut self, msg: GetTask, _: &mut Self::Context) -> Self::Result {
        use self::schema::tasks::dsl::*;

        let conn: &PgConnection = &self.0.get().unwrap();

        let mut t = tasks
            .filter(id.eq(msg.id))
            .load::<models::Task>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error retriving specified task"))?;

        Ok(t.pop().unwrap())
    }
}
