#![allow(proc_macro_derive_resolution_fallback)]

use super::schema::tasks;

#[derive(Queryable, Serialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
    // user ??
}

#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
    pub id: &'a str,
    pub title: &'a str,
    pub completed: bool,
}
