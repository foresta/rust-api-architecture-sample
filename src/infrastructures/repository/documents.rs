use super::super::database::schema::*;
use crate::domains::documents::{Document, DocumentId, DocumentRepository};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use failure::Error;

//
// Entity
//

#[derive(Debug, Clone, Eq, PartialEq, Hash, Insertable)]
#[table_name = "documents"]
pub struct NewDocumentEntity {
    pub title: String,
    pub body: String,
}

impl NewDocumentEntity {
    fn from(model: &Document) -> NewDocumentEntity {
        NewDocumentEntity {
            title: model.title.to_owned(),
            body: model.body.to_owned(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Queryable, Identifiable, AsChangeset)]
#[table_name = "documents"]
pub struct DocumentEntity {
    pub id: u64,
    pub title: String,
    pub body: String,
}

impl DocumentEntity {
    fn from(model: &Document) -> DocumentEntity {
        DocumentEntity {
            id: model.id.get(),
            title: model.title.to_owned(),
            body: model.body.to_owned(),
        }
    }

    fn of(&self) -> Document {
        Document {
            id: DocumentId::new(self.id),
            title: self.title.to_owned(),
            body: self.body.to_owned(),
        }
    }
}

pub struct DocumentRepositoryImpl {
    pub pool: Box<Pool<ConnectionManager<MysqlConnection>>>,
}

impl DocumentRepository for DocumentRepositoryImpl {
    fn find_by_id(&self, document_id: DocumentId) -> Result<Document, Error> {
        use super::super::database::schema::documents::dsl;

        let conn = self.pool.get()?;
        let entity: DocumentEntity = dsl::documents
            .filter(documents::id.eq(document_id.get()))
            .get_result(&conn)?;

        Ok(entity.of())
    }

    fn list(&self) -> Result<Vec<Document>, Error> {
        use super::super::database::schema::documents::dsl;

        let query = dsl::documents.into_boxed();
        let conn = self.pool.get()?;
        let results: Vec<DocumentEntity> = query.limit(100).load(&conn)?;

        Ok(results.into_iter().map(|e| e.of()).collect())
    }

    fn insert(&self, document: &Document) -> Result<(), Error> {
        use super::super::database::schema::documents::dsl;

        let entity = NewDocumentEntity::from(document);
        let conn = self.pool.get().unwrap();
        diesel::insert_into(dsl::documents)
            .values(entity)
            .execute(&conn)?;

        Ok(())
    }

    fn update(&self, document: &Document) -> Result<(), Error> {
        let entity = DocumentEntity::from(document);
        let conn = self.pool.get().unwrap();
        diesel::update(documents::table)
            .set(&entity)
            .execute(&conn)?;

        Ok(())
    }

    fn delete(&self, document: &Document) -> Result<(), Error> {
        let entity = DocumentEntity::from(document);
        let conn = self.pool.get().unwrap();
        diesel::delete(&entity).execute(&conn)?;

        Ok(())
    }
}
