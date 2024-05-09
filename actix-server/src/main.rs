use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    app_name: String,
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix-web"),
                counter: Mutex::new(0),
            }))
            .route("/", web::get().to(index))

    })
    .workers(4)
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name;
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;   
    format!("Hello {app_name}!\n Counter: {counter}\n", app_name=app_name, counter=counter)
}
