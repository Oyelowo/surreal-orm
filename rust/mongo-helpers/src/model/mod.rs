use mongodb::{
    bson::{doc, from_bson, oid::ObjectId, to_bson, Bson, Document},
    options::{FindOneOptions, ReadConcern},
    Collection, Database,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub trait Model
where
    Self: Serialize + DeserializeOwned + Unpin + Send + Sync,
{
    const COLLECTION_NAME: &'static str;

    fn get_collection(db: &Database) -> Collection<Self> {
        db.collection::<Self>(Self::COLLECTION_NAME)
    }

    /// Attempt to serialize the given bson document into an instance of this model.
    // fn instance_from_document(document: Document) -> anyhow::Result<Self> {
    fn from_doc(document: Document) -> anyhow::Result<Self> {
        Ok(from_bson::<Self>(Bson::Document(document))?)
    }

    /// Attempt to serialize an instance of this model into a bson document.
    // fn document_from_instance(&self) -> anyhow::Result<Document> {
    fn to_doc_unchecked(&self) -> Document {
        match to_bson(&self).expect("Problem converting to bson") {
            Bson::Document(doc) => Ok(doc),
            bsn => Err(anyhow::anyhow!("Missing attribute: k")),
        }
        .expect("Not  a document")
    }
    fn to_doc(&self) -> anyhow::Result<Document> {
        match to_bson(&self)? {
            Bson::Document(doc) => Ok(doc),
            bsn => Err(anyhow::anyhow!("Missing attribute: k")),
        }
    }
}
