use mongodb::bson::{self, Document};
use mongodb::Collection;
use serde_json::Value;

pub async fn insert_documents(
    collection: Collection<Document>,
    data: Vec<Value>,
) -> mongodb::error::Result<()> {
    let documents: Vec<Document> = data
        .into_iter()
        .map(|value| bson::to_document(&value).unwrap())
        .collect();

    collection.insert_many(documents).await?;
    Ok(())
}
