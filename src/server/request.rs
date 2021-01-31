use crate::domains::documents::{Document, DocumentId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, Deserialize, Serialize)]
pub struct DocumentRequest {
    title: String,
    body: String,
}

impl DocumentRequest {
    pub fn of(&self) -> Document {
        Document::create(self.title.to_owned(), self.body.to_owned())
    }

    pub fn model(&self, document_id: DocumentId) -> Document {
        Document {
            id: document_id,
            title: self.title.to_owned(),
            body: self.body.to_owned(),
        }
    }
}
