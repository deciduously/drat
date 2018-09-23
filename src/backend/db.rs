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

pub struct GetAllTasks;

impl Message for GetAllTasks {
    type Result = Result<models::TaskList, Error>;
}

/// Message to retrieve a task by id
pub struct GetTask {
    pub id: String,
}

impl Message for GetTask {
    type Result = Result<models::Task, Error>;
}

pub struct ToggleTask {
    pub id: String,
}

impl Message for ToggleTask {
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

        // impl Deref for Pool<ConnectionManager<PgConnection>>!
        let conn: &PgConnection = &self.0.get().unwrap();

        diesel::insert_into(tasks)
            .values(&new_task)
            .execute(conn)
            .map_err(|_| error::ErrorInternalServerError("Error inserting task"))?;

        info!("Inserted Task: {}", &msg.title);

        let mut ts = tasks
            .filter(id.eq(&uuid))
            .load::<models::Task>(conn)
            .map_err(|_| error::ErrorInternalServerError("Error loading new task"))?;

        Ok(ts.pop().unwrap())
    }
}

impl Handler<GetAllTasks> for DbExecutor {
    type Result = Result<models::TaskList, Error>;

    fn handle(&mut self, _msg: GetAllTasks, _: &mut Self::Context) -> Self::Result {
        use self::schema::tasks::dsl::*;

        let conn: &PgConnection = &self.0.get().unwrap();

        let ts = tasks.load::<models::Task>(conn);

        Ok(models::TaskList { tasks: ts.unwrap() })
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
            .map_err(|_| error::ErrorInternalServerError("Error retrieving specified task"))?;

        Ok(t.pop().unwrap())
    }
}

impl Handler<ToggleTask> for DbExecutor {
    type Result = Result<models::Task, Error>;

    fn handle(&mut self, msg: ToggleTask, _: &mut Self::Context) -> Self::Result {
        use self::schema::tasks::dsl::*;

        let conn: &PgConnection = &self.0.get().unwrap();

        // UPDATE
        let current = tasks
            .filter(id.eq(&msg.id))
            .load::<models::Task>(conn)
            .map_err(|_| error::ErrorInternalServerError("Could not find that task"))?
            .pop()
            .unwrap()
            .completed;
        let target = tasks.filter(id.eq(&msg.id));
        let t = diesel::update(target)
            .set(completed.eq(!current))
            .get_result::<models::Task>(conn)
            .unwrap();
        info!("Toggled {} to {}", t.title, t.completed);

        Ok(t)
    }
}
