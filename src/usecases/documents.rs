use crate::domains::documents::{Document, DocumentId, DocumentRepository};
use failure::Error;

pub fn get_document_list(repository: impl DocumentRepository) -> Result<Vec<Document>, Error> {
    repository.list()
}

pub fn get_document(
    repository: impl DocumentRepository,
    document_id: DocumentId,
) -> Result<Document, Error> {
    repository.find_by_id(document_id)
}

pub fn post_document(
    repository: impl DocumentRepository,
    document: &Document,
) -> Result<(), Error> {
    repository.insert(document)
}

pub fn update_document(
    repository: impl DocumentRepository,
    document: &Document,
) -> Result<(), Error> {
    repository.update(document)
}

pub fn delete_document(
    repository: impl DocumentRepository,
    document_id: DocumentId,
) -> Result<(), Error> {
    let document = repository.find_by_id(document_id)?;
    repository.delete(&document)
}
