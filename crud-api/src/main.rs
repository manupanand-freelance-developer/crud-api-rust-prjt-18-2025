use actix_web::{get, web, Responder, http::header, middleware::Logger, App, HttpResponse, HttpServer};
use actix_cors::Cors;
use dotenv::dotenv;
use std::env;
#[get("/health")]
async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", "0.0.1"))
        .body("Health check OK")
}

#[get("/test/{param_value}")]
async fn test_route(path: web::Path<String>) -> impl Responder {
    let param_value = path.into_inner();
    let response_text = format!("Value: {}", param_value);
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body(response_text)
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain; charset=utf-8")
        .body("Got request")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    let port = env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse::<u16>()
    .expect("PORT must be a valid number");

    println!("Starting server on http://0.0.0.0:8080...");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .service(index)       // ✅ register "/"
            .service(health)      // ✅ register "/health"
            .service(test_route)  // ✅ register "/test/{param_value}"
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
