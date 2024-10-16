use mongodb::bson::Document;
use mongodb::Collection;

pub async fn insert_documents(
    collection: Collection<Document>,
    documents: &Vec<Document>,
) -> mongodb::error::Result<()> {
    collection.insert_many(documents).await?;
    Ok(())
}
