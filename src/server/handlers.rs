use super::request::*;
use super::response::*;
use super::RequestContext;
use crate::domains::documents::DocumentId;
use crate::usecases;
use actix_web::{delete, get, post, put, web, web::Json, HttpResponse, Responder};

#[post("/documents")]
async fn post_document(
    data: web::Data<RequestContext>,
    request: Json<DocumentRequest>,
) -> impl Responder {
    match usecases::documents::post_document(data.document_repository(), &request.of()) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[put("/documents/{id}")]
async fn update_document(
    data: web::Data<RequestContext>,
    path_params: web::Path<(u32,)>,
    request: Json<DocumentRequest>,
) -> impl Responder {
    let document_id = DocumentId::new(path_params.into_inner().0.into());
    let document = request.model(document_id);
    match usecases::documents::update_document(data.document_repository(), &document) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[delete("/documents/{id}")]
async fn delete_document(
    data: web::Data<RequestContext>,
    path_params: web::Path<(u32,)>,
) -> impl Responder {
    let document_id = DocumentId::new(path_params.into_inner().0.into());
    match usecases::documents::delete_document(data.document_repository(), document_id) {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[get("/documents/{id}")]
async fn get_document(
    data: web::Data<RequestContext>,
    path_params: web::Path<(u32,)>,
) -> impl Responder {
    let document_id = DocumentId::new(path_params.into_inner().0.into());
    match usecases::documents::get_document(data.document_repository(), document_id) {
        Ok(document) => HttpResponse::Ok().json(DocumentDto::new(&document)),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[get("/documents")]
async fn get_documents(data: web::Data<RequestContext>) -> impl Responder {
    match usecases::documents::get_document_list(data.document_repository()) {
        Ok(documents) => HttpResponse::Ok().json(DocumentListResponse::new(documents)),
        Err(_) => HttpResponse::InternalServerError().json(""),
    }
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
