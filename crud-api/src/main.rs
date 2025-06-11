use actix_web::{http::header, HttpServer};
use sqlx::{PgPool,Postgres,Pool,postgres::PgPoolOptions};
use dotenv::dotenv;


#[actix_web::main]

async fn main()-> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none(){
        std::env::set_var("RUST_LOG","actix_web=info");
    }
    dotenv().ok();
    env_logger::init();

    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool: Pool<Postgres> = match PgPoolOptions::new()
    .max_connections(10)
    .connect(&database_url)
    .await
    {   // like try catch in js
        Ok(pool) => { 
            println!("Connection to db is successful");
            pool  // no semicolon here, so `pool` is returned
        }
        Err(err) => {
            println!("Failed to connect to the database {:?}", err);
            std::process::exit(1);
        }
    };
    println!("Server started succesfully")
    HttpServer::new(move||{
        let cors =Cors::default()
        .allowed_origin("http:://localhost::3000")
    })
}
/*
- setup database 
-setup db pools
-declare routes
-declare cors middleware

*/