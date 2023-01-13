use chrono::prelude::*;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Task<'a> {
    description: &'a str,
    create_time: DateTime<Utc>,
    due_time: DateTime<Utc>,
}

impl<'a> Task<'a> {
    pub fn get_description(&self) -> &'a str {
        self.description
    }

    pub fn set_description(&mut self, description: &'a str) {
        self.description = description
    }

    pub fn get_create_time(&self) -> &DateTime<Utc> {
        &self.create_time
    }

    pub fn set_create_time(&mut self, create_time: DateTime<Utc>) {
        self.create_time = create_time
    }

    pub fn get_due_time(&self) -> &DateTime<Utc> {
        &self.due_time
    }

    pub fn set_due_time(&mut self, due_time: DateTime<Utc>) {
        self.due_time = due_time
    }
}
