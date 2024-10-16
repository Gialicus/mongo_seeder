use fake::faker::address::raw::*;
use fake::faker::company::raw::*;
use fake::faker::currency::raw::*;
use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
use fake::Fake;
use mongodb::bson::oid::ObjectId;
use rand::seq::SliceRandom;
use std::collections::HashMap;

use crate::types::CollectionConfig;

/// Generates a fake value based on the method string.
fn generate_fake_value(
    fake_method: &str,
    ids_pool: &HashMap<String, Vec<ObjectId>>,
) -> mongodb::bson::Bson {
    match fake_method {
        "fake.name.firstName" => {
            mongodb::bson::Bson::String(FirstName(fake::locales::EN).fake::<String>())
        }
        "fake.name.lastName" => {
            mongodb::bson::Bson::String(LastName(fake::locales::EN).fake::<String>())
        }
        "fake.name.fullName" => mongodb::bson::Bson::String(format!(
            "{} {}",
            FirstName(fake::locales::EN).fake::<String>(),
            LastName(fake::locales::EN).fake::<String>()
        )),

        "fake.address.streetName" => {
            mongodb::bson::Bson::String(StreetName(fake::locales::EN).fake::<String>())
        }
        "fake.address.city" => {
            mongodb::bson::Bson::String(CityName(fake::locales::EN).fake::<String>())
        }
        "fake.address.zipCode" => {
            mongodb::bson::Bson::String(ZipCode(fake::locales::EN).fake::<String>())
        }

        "fake.internet.email" => {
            mongodb::bson::Bson::String(FreeEmail(fake::locales::EN).fake::<String>())
        }
        "fake.internet.username" => {
            mongodb::bson::Bson::String(Username(fake::locales::EN).fake::<String>())
        }

        "fake.company.name" => {
            mongodb::bson::Bson::String(CompanyName(fake::locales::EN).fake::<String>())
        }
        "fake.company.suffix" => {
            mongodb::bson::Bson::String(CompanySuffix(fake::locales::EN).fake::<String>())
        }

        "fake.number.u8" => mongodb::bson::Bson::Int32((0..=255).fake::<u8>() as i32),
        "fake.number.i32" => mongodb::bson::Bson::Int32((0..=1000).fake::<i32>()),
        "fake.number.u32" => mongodb::bson::Bson::Int64((0..=1000).fake::<u32>() as i64),
        "fake.number.i64" => mongodb::bson::Bson::Int64((0..=1000).fake::<i64>()),

        "fake.currency.code" => {
            mongodb::bson::Bson::String(CurrencyCode(fake::locales::EN).fake::<String>())
        }
        "fake.currency.name" => {
            mongodb::bson::Bson::String(CurrencyName(fake::locales::EN).fake::<String>())
        }
        "fake.currency.symbol" => {
            mongodb::bson::Bson::String(CurrencySymbol(fake::locales::EN).fake::<String>())
        }

        "fake.chrono.datetime" => mongodb::bson::Bson::String(
            fake::faker::chrono::raw::DateTime(fake::locales::EN).fake::<String>(),
        ),
        "fake.chrono.date" => mongodb::bson::Bson::String(
            fake::faker::chrono::raw::Date(fake::locales::EN).fake::<String>(),
        ),
        "fake.chrono.duration" => mongodb::bson::Bson::String(
            fake::faker::number::raw::NumberWithFormat(fake::locales::EN, "###").fake::<String>(),
        ),

        "fake.random.uuid" => mongodb::bson::Bson::String(fake::uuid::UUIDv4.fake::<String>()),

        method if fake_method.starts_with("ref") => {
            let collection_name = method.split('.').collect::<Vec<&str>>()[1];
            let collection_id = ids_pool.get(collection_name).unwrap();
            let random_id = collection_id.choose(&mut rand::thread_rng()).unwrap();
            mongodb::bson::Bson::ObjectId(*random_id)
        }

        _ => mongodb::bson::Bson::String("unsupported_method".to_string()),
    }
}

/// Generates data based on a schema, handling scalar fields, arrays, and nested objects.
pub fn generate_data(
    schema: &HashMap<String, serde_json::Value>,
    number_of_children: usize,
    ids_pool: &HashMap<String, Vec<ObjectId>>,
) -> mongodb::bson::Bson {
    let mut document = mongodb::bson::Document::new();

    for (key, value) in schema.iter() {
        let generated_value = match value {
            // Scalar value represented as a string
            serde_json::Value::String(fake_method) => generate_fake_value(fake_method, ids_pool),

            // Array of values or objects
            serde_json::Value::Array(array_spec) => {
                if array_spec.is_empty() {
                    mongodb::bson::Bson::Array(vec![]) // Empty array if the schema contains no information
                } else if let Some(serde_json::Value::String(fake_method)) = array_spec.get(0) {
                    // Array of primitive values (e.g., array of strings)
                    let generated_array: Vec<mongodb::bson::Bson> = (0..number_of_children)
                        .map(|_| generate_fake_value(fake_method, ids_pool))
                        .collect();
                    mongodb::bson::Bson::Array(generated_array)
                } else if let Some(serde_json::Value::Object(object_schema)) = array_spec.get(0) {
                    // Array of objects: each object follows a schema
                    let generated_array: Vec<mongodb::bson::Bson> = (0..number_of_children)
                        .map(|_| {
                            generate_data(
                                &object_schema
                                    .iter()
                                    .map(|(k, v)| (k.clone(), v.clone()))
                                    .collect(),
                                number_of_children,
                                ids_pool,
                            )
                        })
                        .collect();
                    mongodb::bson::Bson::Array(generated_array)
                } else {
                    mongodb::bson::Bson::Array(vec![])
                }
            }
            // Nested objects
            serde_json::Value::Object(nested_schema) => {
                let nested_document = generate_data(
                    &nested_schema
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                    number_of_children,
                    ids_pool,
                );
                mongodb::bson::Bson::Document(nested_document.as_document().unwrap().clone())
            }
            _ => mongodb::bson::Bson::Null,
        };

        document.insert(key.clone(), generated_value);
    }

    mongodb::bson::Bson::Document(document)
}

/// Generates a pool of IDs for each collection specified in the configuration file.
pub fn generate_ids_pool(
    collections_config: &Vec<CollectionConfig>,
) -> HashMap<String, Vec<ObjectId>> {
    let mut ids_pool = HashMap::new();

    for collection_config in collections_config {
        let mut ids = Vec::new();
        let number_of_items = collection_config.number_of_items as usize;
        for _ in 0..number_of_items {
            ids.push(ObjectId::new());
        }
        ids_pool.insert(collection_config.name.clone(), ids);
    }

    ids_pool
}
