mod db;
use db::User;

use topcoat::{
    Result,
    context::Cx,
    router::{
        Form, IntoResponse, Response, Router, RouterBuilderDiscoverExt, page, route, see_other,
    },
    view::{component, view},
};

#[tokio::main]
async fn main() {
    let db = db::connect().await.unwrap();

    topcoat::start(Router::builder().app_context(db).discover().build())
        .await
        .unwrap();
}

#[page("/")]
async fn home(cx: &Cx) -> Result {
    let mut db = db::get(cx).clone();
    let users = User::all().exec(&mut db).await.unwrap();

    view! {
        <!DOCTYPE html>
        <html>
            <head>
                <title>"Hello world"</title>
                topcoat::dev::script()
            </head>
            <body>
                hello(name: "Ruan")
                for user in users {
                    hello(name: &user.name)
                }
                user_form()
            </body>
        </html>
    }
}

#[derive(Debug, serde::Deserialize)]
struct CreateUserForm {
    name: String,
    email: String,
}

#[route(POST "/create-user")]
async fn create_user(cx: &Cx, Form(input): Form<CreateUserForm>) -> Result<Response> {
    let mut db = db::get(cx).clone();

    toasty::create!(User {
        name: input.name,
        email: input.email,
    })
    .exec(&mut db)
    .await
    .unwrap();

    Ok(see_other("/").into_response(cx)?)
}

#[component]
async fn user_form() -> Result {
    view! {
        <form method="POST" action="/create-user">
            <input type="text" name="name" placeholder="Seu nome" required="true"/>
            <input type="email" name="email" placeholder="Seu email" required="true" />
            <button type="submit">"Cadastrar"</button>
        </form>
    }
}

#[component]
async fn hello(name: &str) -> Result {
    view! {
        <h1>"Hello, " (name) "!"</h1>
    }
}
