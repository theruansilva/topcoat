use toasty::{Db, Deferred, Model, models};
use topcoat::context::{Cx, app_context};

#[derive(Debug, Model)]
pub struct User {
    #[key]
    #[auto]
    pub id: u64,
    pub name: String,

    #[unique]
    pub email: String,

    #[has_many]
    pub todos: Deferred<Vec<Todo>>,
}

#[derive(Debug, Model)]
pub struct Todo {
    #[key]
    #[auto]
    pub id: u64,

    pub title: String,
    pub done: bool,

    #[index]
    pub user_id: u64,
    #[belongs_to(key = user_id, references = id)]
    pub user: Deferred<User>,
}

pub async fn connect() -> Result<Db, Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/toasty_dev".to_string());

    println!("Conectando ao PostgreSQL em: {}", database_url);

    let driver = toasty_driver_postgresql::PostgreSQL::new(&database_url)?;

    let db = Db::builder()
        .models(models!(User, Todo))
        .build(driver)
        .await?;

    Ok(db)
}

pub fn get(cx: &Cx) -> &Db {
    app_context::<Db>(cx)
}
