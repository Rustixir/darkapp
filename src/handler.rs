use actix_web::{Responder, post, HttpResponse, web, get};

use crate::{movie::Movie, repo::Repository, dto};






#[post("/add")]
pub async fn add(db: web::Data<Repository>, movie: web::Json<Movie>) -> impl Responder {
    let id = db.next_id();
    match db.insert(id, movie.into_inner()).await {
        Ok(_) => HttpResponse::Ok().body("success"),
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }
}


#[get("/get/{id}")]
pub async fn get(db: web::Data<Repository>, id: web::Path<u64>) -> impl Responder {
    match db.lookup(id.into_inner()) {
        Ok(res) => {
            match res {
                Some(rf) => HttpResponse::Ok().json(dto::Movie::new(rf)),
                None => HttpResponse::NotFound().body("")                
            }
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }
}


#[get("/get-by-name/{name}")]
pub async fn get_by_name(db: web::Data<Repository>, title: web::Path<String>) -> impl Responder {
    match db.lookup_by_index(&title.into_inner()) {
        Ok(res) => {
            match res {
                Some(rf) => HttpResponse::Ok().json(dto::Movie::new(rf)),
                None => HttpResponse::Ok().body("")
            }
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }
}

#[get("/gets-by-tag/{tag}")]
pub async fn gets_by_tag(db: web::Data<Repository>, tag: web::Path<String>) -> impl Responder {
    match db.lookup_by_tag(&tag.into_inner()) {
        Ok(res) => {
            HttpResponse::Ok().json(dto::Movie::new_list(res))
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }
}


#[get("/search/{text}")]
pub async fn find(db: web::Data<Repository>, text: web::Path<String>) -> impl Responder {
    match db.search(text.into_inner()).await {
        Ok(res) => {
            HttpResponse::Ok().json(dto::Movie::new_list(res))
        }
        Err(e) => HttpResponse::BadRequest().body(e.to_string())
    }
}






