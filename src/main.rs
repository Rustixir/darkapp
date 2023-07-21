

pub mod movie;
pub mod repo;
pub mod handler;
pub mod dto;

use actix_web::{HttpServer, App, web};
use repo::Repository;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let repo =  Repository::new().await;
    let counter = web::Data::new(report);

    HttpServer::new(move || {
        // move counter into the closure
        App::new()
            .app_data(counter.clone()) 
            .service(
                web::scope("/movies")
                    .service(handler::add)
                    .service(handler::get)
                    .service(handler::get_by_name)
                    .service(handler::gets_by_tag)
                    .service(handler::search)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}