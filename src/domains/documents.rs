use super::Id;
use failure::Error;

pub type DocumentId = Id<Document>;

#[derive(Debug, Clone)]
pub struct Document {
    pub id: DocumentId,
    pub title: String,
    pub body: String,
}

impl Document {
    pub fn create(title: String, body: String) -> Self {
        Self {
            id: Default::default(),
            title: title,
            body: body,
        }
    }
}

pub trait DocumentRepository {
    fn find_by_id(&self, document_id: DocumentId) -> Result<Document, Error>;
    fn list(&self) -> Result<Vec<Document>, Error>;
    fn insert(&self, document: &Document) -> Result<(), Error>;
    fn update(&self, document: &Document) -> Result<(), Error>;
    fn delete(&self, document: &Document) -> Result<(), Error>;
}
