use actix_web::{web, HttpResponse, Responder};
use crate::models::post::{CreatePost, UpdatePost};
use crate::services::post_service::PostService;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/posts")
            .route(web::post().to(create_post))
            .route(web::get().to(get_all_posts)),
    );
    cfg.service(
        web::resource("/posts/{id}")
            .route(web::get().to(get_post))
            .route(web::put().to(update_post))
            .route(web::delete().to(delete_post)),
    );

}

async fn create_post(post: web::Json<CreatePost>) -> impl Responder {
    match PostService::create_post(post.into_inner()) {
        Ok(created_post) => HttpResponse::Ok().json(created_post),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn get_post(path: web::Path<i32>) -> impl Responder {
    match PostService::read_post(path.into_inner()) {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn update_post(post: web::Json<UpdatePost>) -> impl Responder {
    match PostService::update_post(post.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn delete_post(path: web::Path<i32>) -> impl Responder {
    match PostService::delete_post(path.into_inner()) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn get_all_posts() -> impl Responder {
    match PostService::get_all_posts() {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
