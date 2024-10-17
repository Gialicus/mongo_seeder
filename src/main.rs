use std::collections::HashMap;

use clap::Parser;
use error::AppError;
use insert::insert_documents;
use mongodb::{
    bson::{oid::ObjectId, Document},
    Client,
};
use types::Cli;

mod error;
mod generate_data;
mod insert;
mod read_config;
mod types;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // Parse command line arguments
    let args = Cli::parse();

    // Read the JSON configuration file
    let config = read_config::read_config(&args.config).map_err(|e| {
        AppError::ConfigError(format!("Error reading the configuration file: {}", e))
    })?;

    // Connect to MongoDB
    let client = Client::with_uri_str(&args.url).await?;
    let database = client.database(&config.db);

    // Generate a pool of IDs for the collections
    let ids_pool = generate_data::generate_ids_pool(&config.collections);

    // Process each collection
    for collection_config in config.collections {
        process_collection(&database, &collection_config, &ids_pool).await?;
    }

    Ok(())
}

async fn process_collection(
    database: &mongodb::Database,
    collection_config: &types::CollectionConfig,
    ids_pool: &HashMap<String, Vec<ObjectId>>,
) -> Result<(), AppError> {
    let collection_name = collection_config.name.as_str();
    let collection = database.collection::<Document>(collection_name);

    // Generate documents for the collection
    let documents = generate_documents(collection_config, ids_pool)?;

    // Insert the documents into the database
    insert_documents(collection, &documents).await?;
    println!(
        "Inserted {} documents into the collection '{}'",
        collection_config.number_of_items, collection_name
    );

    Ok(())
}

fn generate_documents(
    collection_config: &types::CollectionConfig,
    ids_pool: &HashMap<String, Vec<ObjectId>>,
) -> Result<Vec<Document>, AppError> {
    let number_of_items = collection_config.number_of_items as usize;
    let schema = &collection_config.schema;

    let mut documents = Vec::new();
    for i in 0..number_of_items {
        // Generate a single document based on the schema
        let mut document =
            generate_data::generate_data(schema, collection_config.number_of_children, ids_pool)?;
        // Assign a unique ID to the document
        let collection_id = ids_pool.get(&collection_config.name).ok_or_else(|| {
            AppError::ConfigError(format!(
                "Collection ID not found for {}",
                &collection_config.name
            ))
        })?;
        let doc = document.as_document_mut().ok_or_else(|| {
            AppError::ConfigError("Failed to convert Bson to Document".to_string())
        })?;
        doc.insert("_id".to_string(), collection_id[i].clone());
        documents.push(doc.clone());
    }

    Ok(documents)
}
