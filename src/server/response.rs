use crate::domains::documents::Document;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct DocumentListResponse {
    documents: Vec<DocumentDto>,
}

impl DocumentListResponse {
    pub fn new(docs: Vec<Document>) -> DocumentListResponse {
        DocumentListResponse {
            documents: docs.iter().map(|d| DocumentDto::new(&d)).collect(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DocumentDto {
    id: u64,
    title: String,
    body: String,
}

impl DocumentDto {
    pub fn new(model: &Document) -> DocumentDto {
        DocumentDto {
            id: model.id.get(),
            title: model.title.to_owned(),
            body: model.body.to_owned(),
        }
    }
}
