use clap::Parser;
use insert::insert_documents;
use mongodb::{bson::Document, Client};
use types::Cli;

mod generate_data;
mod insert;
mod read_config;
mod types;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Gestione della CLI con clap
    let args = Cli::parse();

    // Leggi il file di configurazione JSON
    let config = read_config::read_config(&args.config)
        .expect("Errore nella lettura del file di configurazione");

    // Connettiti a MongoDB
    let client = Client::with_uri_str(&config.url).await?;
    let database = client.database(&config.db);

    // Cicla attraverso le collezioni specificate
    for collection_config in config.collections {
        let collection_name = collection_config.name.as_str();
        let collection = database.collection::<Document>(collection_name);

        // Generazione dei documenti per ogni collezione
        let number_of_items = collection_config.number_of_items as usize;
        let schema = collection_config.schema;

        let mut documents = Vec::new();
        for _ in 0..number_of_items {
            let document =
                generate_data::generate_data(&schema, collection_config.number_of_children);
            documents.push(document);
        }

        // Inserisci i documenti nel database
        insert_documents(collection, documents).await?;
        println!(
            "Inseriti {} documenti nella collezione '{}'",
            number_of_items, collection_name
        );
    }

    Ok(())
}
