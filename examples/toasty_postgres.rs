use toasty::{Db, Deferred, Model, create, models, query, update};

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/toasty_dev".to_string());

    println!("Conectando ao PostgreSQL em: {}", database_url);

    let driver = toasty_driver_postgresql::PostgreSQL::new(&database_url)?;

    let mut db = Db::builder()
        .models(models!(User, Todo))
        .build(driver)
        .await?;

    println!("Limpando tabelas antigas e criando novo schema no PostgreSQL (push_schema)...");
    let _ = toasty::sql::statement("DROP TABLE IF EXISTS todos, users CASCADE")
        .exec(&mut db)
        .await;
    db.push_schema().await?;

    println!("\n--- 1. Criando Usuário com tarefas (Relacionamento HasMany) ---");
    let user = create!(User {
        name: "Dev User",
        email: "dev@example.com",
        todos: [
            { title: "Configurar Docker & PostgreSQL", done: true },
            { title: "Testar Toasty ORM", done: false },
        ],
    })
    .exec(&mut db)
    .await?;

    println!("Usuário criado com sucesso (ID: {})", user.id);

    println!("\n--- 2. Buscando Usuário por E-mail ---");
    let found_user = User::get_by_email(&mut db, "dev@example.com").await?;
    println!(
        "Usuário encontrado: ID = {}, Nome = {}",
        found_user.id, found_user.name
    );

    println!("\n--- 3. Carregando Todos do Usuário ---");
    let todos = found_user.todos().exec(&mut db).await?;
    for t in &todos {
        println!(
            "  [Todo ID: {}] - {} (Concluído: {})",
            t.id, t.title, t.done
        );
    }

    println!("\n--- 4. Atualizando um Todo usando a macro update! ---");
    if let Some(mut first_todo) = todos.into_iter().find(|t| !t.done) {
        println!("Marcando todo '{}' como concluído...", first_todo.title);
        update!(first_todo { done: true }).exec(&mut db).await?;
        println!("Todo atualizado!");
    }

    println!("\n--- 5. Usando a Macro query! para Filtrar ---");
    let completed_todos = query!(Todo FILTER .done == true).exec(&mut db).await?;
    println!("Todos concluídos encontrados via query!:");
    for t in completed_todos {
        println!("  - {}", t.title);
    }

    println!("\n--- 6. Removendo Usuário e dados de teste ---");
    let user_to_del = User::get_by_email(&mut db, "dev@example.com").await?;
    user_to_del.delete().exec(&mut db).await?;
    println!("Usuário e seus relacionamentos removidos.");

    println!("\nTeste do Toasty ORM com PostgreSQL em Docker concluído com sucesso!");

    Ok(())
}
