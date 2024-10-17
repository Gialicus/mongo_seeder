use fake::faker::address::raw::*;
use fake::faker::boolean::raw::*;
use fake::faker::company::raw::*;
use fake::faker::creditcard::raw::*;
use fake::faker::currency::raw::*;
use fake::faker::filesystem::raw::*;
use fake::faker::internet::raw::*;
use fake::faker::job::raw::{Title as JobTitle, *};
use fake::faker::lorem::raw::*;
use fake::faker::name::raw::*;
use fake::Fake;
use mongodb::bson::{oid::ObjectId, Bson};
use rand::seq::SliceRandom;
use std::collections::HashMap;

use crate::types::CollectionConfig;

/// Generates a fake value based on the method string.
fn generate_fake_value(fake_method: &str, ids_pool: &HashMap<String, Vec<ObjectId>>) -> Bson {
    match fake_method {
        // Address
        "fake.address.buildingNumber" => {
            Bson::String(BuildingNumber(fake::locales::EN).fake::<String>())
        }
        "fake.address.cityName" => Bson::String(CityName(fake::locales::EN).fake::<String>()),
        "fake.address.cityPrefix" => Bson::String(CityPrefix(fake::locales::EN).fake::<String>()),
        "fake.address.citySuffix" => Bson::String(CitySuffix(fake::locales::EN).fake::<String>()),
        "fake.address.countryCode" => Bson::String(CountryCode(fake::locales::EN).fake::<String>()),
        "fake.address.countryName" => Bson::String(CountryName(fake::locales::EN).fake::<String>()),
        "fake.address.latitude" => Bson::String(Latitude(fake::locales::EN).fake::<String>()),
        "fake.address.longitude" => Bson::String(Longitude(fake::locales::EN).fake::<String>()),
        "fake.address.postCode" => Bson::String(PostCode(fake::locales::EN).fake::<String>()),
        "fake.address.secondaryAddress" => {
            Bson::String(SecondaryAddress(fake::locales::EN).fake::<String>())
        }
        "fake.address.stateAbbr" => Bson::String(StateAbbr(fake::locales::EN).fake::<String>()),
        "fake.address.stateName" => Bson::String(StateName(fake::locales::EN).fake::<String>()),
        "fake.address.streetName" => Bson::String(StreetName(fake::locales::EN).fake::<String>()),
        "fake.address.streetSuffix" => {
            Bson::String(StreetSuffix(fake::locales::EN).fake::<String>())
        }
        "fake.address.timeZone" => Bson::String(TimeZone(fake::locales::EN).fake::<String>()),
        "fake.address.zipCode" => Bson::String(ZipCode(fake::locales::EN).fake::<String>()),

        // Boolean
        "fake.boolean.boolean" => Bson::Boolean(Boolean(fake::locales::EN, 1).fake()),

        // Chrono
        "fake.chrono.date" => {
            Bson::String(fake::faker::chrono::raw::Date(fake::locales::EN).fake::<String>())
        }
        "fake.chrono.datetime" => {
            Bson::String(fake::faker::chrono::raw::DateTime(fake::locales::EN).fake::<String>())
        }
        "fake.chrono.duration" => Bson::String(
            fake::faker::number::raw::NumberWithFormat(fake::locales::EN, "###").fake::<String>(),
        ),
        "fake.chrono.time" => {
            Bson::String(fake::faker::chrono::raw::Time(fake::locales::EN).fake::<String>())
        }

        // Company
        "fake.company.bs" => Bson::String(Bs(fake::locales::EN).fake::<String>()),
        "fake.company.buzzword" => Bson::String(Buzzword(fake::locales::EN).fake::<String>()),
        "fake.company.catchPhrase" => Bson::String(CatchPhrase(fake::locales::EN).fake::<String>()),
        "fake.company.name" => Bson::String(CompanyName(fake::locales::EN).fake::<String>()),
        "fake.company.profession" => Bson::String(Profession(fake::locales::EN).fake::<String>()),
        "fake.company.suffix" => Bson::String(Suffix(fake::locales::EN).fake::<String>()),
        "fake.company.industry" => Bson::String(Industry(fake::locales::EN).fake::<String>()),

        // CreditCard
        "fake.creditCard.number" => {
            Bson::String(CreditCardNumber(fake::locales::EN).fake::<String>())
        }

        // Currency
        "fake.currency.code" => Bson::String(CurrencyCode(fake::locales::EN).fake::<String>()),
        "fake.currency.name" => Bson::String(CurrencyName(fake::locales::EN).fake::<String>()),
        "fake.currency.symbol" => Bson::String(CurrencySymbol(fake::locales::EN).fake::<String>()),

        // FileSystem
        "fake.fileSystem.extension" => {
            Bson::String(FileExtension(fake::locales::EN).fake::<String>())
        }
        "fake.fileSystem.fileName" => Bson::String(FileName(fake::locales::EN).fake::<String>()),
        "fake.fileSystem.filePath" => Bson::String(FilePath(fake::locales::EN).fake::<String>()),
        "fake.fileSystem.mimeType" => Bson::String(MimeType(fake::locales::EN).fake::<String>()),

        // Internet
        "fake.internet.domainSuffix" => {
            Bson::String(DomainSuffix(fake::locales::EN).fake::<String>())
        }
        "fake.internet.freeEmail" => Bson::String(FreeEmail(fake::locales::EN).fake::<String>()),
        "fake.internet.ipV4" => Bson::String(IPv4(fake::locales::EN).fake::<String>()),
        "fake.internet.ipV6" => Bson::String(IPv6(fake::locales::EN).fake::<String>()),
        "fake.internet.password" => {
            Bson::String(Password(fake::locales::EN, 8..12).fake::<String>())
        }
        "fake.internet.email" => Bson::String(FreeEmail(fake::locales::EN).fake::<String>()),
        "fake.internet.username" => Bson::String(Username(fake::locales::EN).fake::<String>()),

        // Job
        "fake.job.field" => Bson::String(Field(fake::locales::EN).fake::<String>()),
        "fake.job.position" => Bson::String(Position(fake::locales::EN).fake::<String>()),
        "fake.job.seniority" => Bson::String(Seniority(fake::locales::EN).fake::<String>()),
        "fake.job.title" => Bson::String(JobTitle(fake::locales::EN).fake::<String>()),

        // Lorem
        "fake.lorem.sentence" => Bson::String(Sentence(fake::locales::EN, 1..5).fake::<String>()),
        "fake.lorem.word" => Bson::String(Word(fake::locales::EN).fake::<String>()),
        "fake.lorem.paragraph" => Bson::String(Paragraph(fake::locales::EN, 1..5).fake::<String>()),

        // Name
        "fake.name.firstName" => Bson::String(FirstName(fake::locales::EN).fake::<String>()),
        "fake.name.lastName" => Bson::String(LastName(fake::locales::EN).fake::<String>()),
        "fake.name.fullName" => Bson::String(format!(
            "{} {}",
            FirstName(fake::locales::EN).fake::<String>(),
            LastName(fake::locales::EN).fake::<String>()
        )),

        "fake.number.u8" => Bson::Int32((0..=255).fake::<u8>() as i32),
        "fake.number.i32" => Bson::Int32((0..=1000).fake::<i32>()),
        "fake.number.u32" => Bson::Int64((0..=1000).fake::<u32>() as i64),
        "fake.number.i64" => Bson::Int64((0..=1000).fake::<i64>()),
        "fake.number.u64" => Bson::Int64((0..=1000).fake::<u64>() as i64),
        "fake.number.f32" => Bson::Double((0.0..1000.0).fake::<f32>() as f64),
        "fake.number.f64" => Bson::Double((0.0..=1000.0).fake::<f64>()),

        "fake.random.uuid" => Bson::String(fake::uuid::UUIDv4.fake::<String>()),

        method if fake_method.starts_with("ref") => {
            let collection_name = method.split('.').collect::<Vec<&str>>()[1];
            let collection_id = ids_pool.get(collection_name).unwrap();
            let random_id = collection_id.choose(&mut rand::thread_rng()).unwrap();
            Bson::ObjectId(*random_id)
        }

        method if fake_method.starts_with("from") => {
            let sequence = method.split('.').collect::<Vec<&str>>()[1];
            let values = sequence.split('|').collect::<Vec<&str>>();
            let random_value = values.choose(&mut rand::thread_rng()).unwrap();
            Bson::String(random_value.to_string())
        }

        _ => Bson::String("unsupported_method".to_string()),
    }
}

/// Generates data based on a schema, handling scalar fields, arrays, and nested objects.
pub fn generate_data(
    schema: &HashMap<String, serde_json::Value>,
    number_of_children: usize,
    ids_pool: &HashMap<String, Vec<ObjectId>>,
) -> Bson {
    let mut document = mongodb::bson::Document::new();

    for (key, value) in schema.iter() {
        let generated_value = match value {
            // Scalar value represented as a string
            serde_json::Value::String(fake_method) => generate_fake_value(fake_method, ids_pool),

            // Array of values or objects
            serde_json::Value::Array(array_spec) => {
                if array_spec.is_empty() {
                    Bson::Array(vec![]) // Empty array if the schema contains no information
                } else if let Some(serde_json::Value::String(fake_method)) = array_spec.get(0) {
                    // Array of primitive values (e.g., array of strings)
                    let generated_array: Vec<Bson> = (0..number_of_children)
                        .map(|_| generate_fake_value(fake_method, ids_pool))
                        .collect();
                    Bson::Array(generated_array)
                } else if let Some(serde_json::Value::Object(object_schema)) = array_spec.get(0) {
                    // Array of objects: each object follows a schema
                    let generated_array: Vec<Bson> = (0..number_of_children)
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
                    Bson::Array(generated_array)
                } else {
                    Bson::Array(vec![])
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
                Bson::Document(nested_document.as_document().unwrap().clone())
            }
            _ => Bson::Null,
        };

        document.insert(key.clone(), generated_value);
    }

    Bson::Document(document)
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_generate_fake_value() {
        let ids_pool = HashMap::new();

        assert!(matches!(
            generate_fake_value("fake.name.firstName", &ids_pool),
            Bson::String(_)
        ));
        assert!(matches!(
            generate_fake_value("fake.address.city", &ids_pool),
            Bson::String(_)
        ));
        assert!(matches!(
            generate_fake_value("fake.number.i32", &ids_pool),
            Bson::Int32(_)
        ));
        assert!(matches!(
            generate_fake_value("fake.currency.code", &ids_pool),
            Bson::String(_)
        ));
        assert!(matches!(
            generate_fake_value("fake.random.uuid", &ids_pool),
            Bson::String(_)
        ));
    }

    #[test]
    fn test_generate_data() {
        let schema = json!({
            "first_name": "fake.name.firstName",
            "last_name": "fake.name.lastName",
            "address": {
                "street": "fake.address.streetName",
                "city": "fake.address.city"
            },
            "emails": ["fake.internet.email"],
            "numbers": ["fake.number.i32"]
        });

        let ids_pool = HashMap::new();
        let schema_map: HashMap<String, serde_json::Value> =
            schema.as_object().unwrap().clone().into_iter().collect();
        let generated_data = generate_data(&schema_map, 3, &ids_pool);

        assert!(generated_data
            .as_document()
            .unwrap()
            .contains_key("first_name"));
        assert!(generated_data
            .as_document()
            .unwrap()
            .contains_key("last_name"));
        assert!(generated_data
            .as_document()
            .unwrap()
            .contains_key("address"));
        assert!(generated_data.as_document().unwrap().contains_key("emails"));
        assert!(generated_data
            .as_document()
            .unwrap()
            .contains_key("numbers"));
    }

    #[test]
    fn test_generate_ids_pool() {
        let collections_config = vec![
            CollectionConfig {
                name: "users".to_string(),
                number_of_items: 5,
                number_of_children: 2,  // Add appropriate value
                schema: HashMap::new(), // Add appropriate schema
            },
            CollectionConfig {
                name: "orders".to_string(),
                number_of_items: 3,
                number_of_children: 2,  // Add appropriate value
                schema: HashMap::new(), // Add appropriate schema
            },
        ];

        let ids_pool = generate_ids_pool(&collections_config);

        assert_eq!(ids_pool.get("users").unwrap().len(), 5);
        assert_eq!(ids_pool.get("orders").unwrap().len(), 3);
    }
}
