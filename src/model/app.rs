use deadpool_postgres::Pool;
use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    db: Pool,
    templates: Tera,
}

impl AppState {
    pub fn new(db: Pool, templates: Tera) -> Self {
        Self { db, templates }
    }

    pub fn db(&self) -> &Pool {
        &self.db
    }

    pub fn templates(&self) -> &Tera {
        &self.templates
    }
}
