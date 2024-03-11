use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
mod api_executer;
use api_executer::ApiExecuter;

pub struct AppState {
    domain: String,
    executer: ApiExecuter,
}

#[get("/")]
async fn main_page() -> impl Responder {
    HttpResponse::Ok().body("Main page")
}

#[post("/api")]
async fn api_call(state: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body("Api")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().app_data(web::Data::new(AppState {
                domain: String::from("localhost:8080"),
                executer: ApiExecuter {},
            }))
            .service(main_page)
            .service(api_call)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}