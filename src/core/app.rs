use crate::core::common::*;
use crate::core::db::*;

pub struct App {
    db: DB,
}

impl App {
    pub fn new(db: DB) -> Self {
        App { db }
    }

    pub fn update(&self, update: Update) -> AppResult<()> {
        self.db.update(update)
    }

    pub fn list_updates(&self) -> AppResult<Vec<Update>> {
        self.db.list_updates()
    }
}
