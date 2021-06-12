use super::api_error::ApiError;
use super::model::{Charactor, Charactors, Names, Titles};
use actix_web::{delete, get, post, put, web, HttpResponse};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Req<T> {
    search: T,
}

#[derive(Deserialize, Clone)]
struct CharactorUpdate {
    id: i32,
    name: String,
    title: String,
    quote: String,
}

#[get("/charactors")]
async fn view_all() -> Result<HttpResponse, ApiError> {
    let charactors = Charactors::view_all()?;
    Ok(HttpResponse::Ok().json(charactors))
}

#[get("/charactors/titles/list")]
async fn view_titlelist() -> Result<HttpResponse, ApiError> {
    let titles = Titles::view_titles()?;
    Ok(HttpResponse::Ok().json(titles))
}

#[get("/charactors/names/list")]
async fn view_names() -> Result<HttpResponse, ApiError> {
    let names = Names::view_names()?;
    Ok(HttpResponse::Ok().json(names))
}

#[post("charactors/id")]
async fn search_by_id(id: web::Json<Req<i32>>) -> Result<HttpResponse, ApiError> {
    let charactor = Charactors::search_by_id(id.into_inner().search)?;
    Ok(HttpResponse::Ok().json(charactor))
}

#[post("charactors/names")]
async fn search_by_name(name: web::Json<Req<String>>) -> Result<HttpResponse, ApiError> {
    let charactors = Charactors::search_by_name(name.into_inner().search)?;
    Ok(HttpResponse::Ok().json(charactors))
}

#[post("charactors/titles")]
async fn search_by_title(title: web::Json<Req<String>>) -> Result<HttpResponse, ApiError> {
    let charactors = Charactors::seach_by_title(title.into_inner().search)?;
    Ok(HttpResponse::Ok().json(charactors))
}

#[post("/charactors")]
async fn create(charactor: web::Json<Charactor>) -> Result<HttpResponse, ApiError> {
    let charactor = Charactors::create(charactor.into_inner())?;
    Ok(HttpResponse::Ok().json(charactor))
}

#[put("charactors")]
async fn update(charactor_update: web::Json<CharactorUpdate>) -> Result<HttpResponse, ApiError> {
    let data = charactor_update.into_inner();
    let charactor = Charactor {
        name: data.name,
        title: data.title,
        quote: data.quote,
    };
    let charactor = Charactors::update(data.id, charactor)?;
    Ok(HttpResponse::Ok().json(charactor))
}

#[delete("/charactors")]
async fn delete(id: web::Json<Req<i32>>) -> Result<HttpResponse, ApiError> {
    let delete = Charactors::delete(id.into_inner().search)?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": delete })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(view_all);
    config.service(view_titlelist);
    config.service(view_names);
    config.service(search_by_id);
    config.service(search_by_name);
    config.service(search_by_title);
    config.service(create);
    config.service(update);
    config.service(delete);
}
