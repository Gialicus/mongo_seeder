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
        // Address
        "fake.address.buildingNumber" => {
            mongodb::bson::Bson::String(BuildingNumber(fake::locales::EN).fake::<String>())
        }
        "fake.address.cityName" => {
            mongodb::bson::Bson::String(CityName(fake::locales::EN).fake::<String>())
        }
        "fake.address.cityPrefix" => {
            mongodb::bson::Bson::String(CityPrefix(fake::locales::EN).fake::<String>())
        }
        "fake.address.citySuffix" => {
            mongodb::bson::Bson::String(CitySuffix(fake::locales::EN).fake::<String>())
        }
        "fake.address.countryCode" => {
            mongodb::bson::Bson::String(CountryCode(fake::locales::EN).fake::<String>())
        }
        "fake.address.countryName" => {
            mongodb::bson::Bson::String(CountryName(fake::locales::EN).fake::<String>())
        }
        "fake.address.latitude" => {
            mongodb::bson::Bson::String(Latitude(fake::locales::EN).fake::<String>())
        }
        "fake.address.longitude" => {
            mongodb::bson::Bson::String(Longitude(fake::locales::EN).fake::<String>())
        }
        "fake.address.postCode" => {
            mongodb::bson::Bson::String(PostCode(fake::locales::EN).fake::<String>())
        }
        "fake.address.secondaryAddress" => {
            mongodb::bson::Bson::String(SecondaryAddress(fake::locales::EN).fake::<String>())
        }
        "fake.address.stateAbbr" => {
            mongodb::bson::Bson::String(StateAbbr(fake::locales::EN).fake::<String>())
        }
        "fake.address.stateName" => {
            mongodb::bson::Bson::String(StateName(fake::locales::EN).fake::<String>())
        }
        "fake.address.streetName" => {
            mongodb::bson::Bson::String(StreetName(fake::locales::EN).fake::<String>())
        }
        "fake.address.streetSuffix" => {
            mongodb::bson::Bson::String(StreetSuffix(fake::locales::EN).fake::<String>())
        }
        "fake.address.timeZone" => {
            mongodb::bson::Bson::String(TimeZone(fake::locales::EN).fake::<String>())
        }
        "fake.address.zipCode" => {
            mongodb::bson::Bson::String(ZipCode(fake::locales::EN).fake::<String>())
        }

        // Boolean
        "fake.boolean.boolean" => {
            mongodb::bson::Bson::Boolean(Boolean(fake::locales::EN, 1).fake())
        }

        // Chrono
        "fake.chrono.date" => mongodb::bson::Bson::String(
            fake::faker::chrono::raw::Date(fake::locales::EN).fake::<String>(),
        ),
        "fake.chrono.datetime" => mongodb::bson::Bson::String(
            fake::faker::chrono::raw::DateTime(fake::locales::EN).fake::<String>(),
        ),
        "fake.chrono.duration" => mongodb::bson::Bson::String(
            fake::faker::number::raw::NumberWithFormat(fake::locales::EN, "###").fake::<String>(),
        ),
        "fake.chrono.time" => mongodb::bson::Bson::String(
            fake::faker::chrono::raw::Time(fake::locales::EN).fake::<String>(),
        ),

        // Company
        "fake.company.bs" => mongodb::bson::Bson::String(Bs(fake::locales::EN).fake::<String>()),
        "fake.company.buzzword" => {
            mongodb::bson::Bson::String(Buzzword(fake::locales::EN).fake::<String>())
        }
        "fake.company.catchPhrase" => {
            mongodb::bson::Bson::String(CatchPhrase(fake::locales::EN).fake::<String>())
        }
        "fake.company.name" => {
            mongodb::bson::Bson::String(CompanyName(fake::locales::EN).fake::<String>())
        }
        "fake.company.profession" => {
            mongodb::bson::Bson::String(Profession(fake::locales::EN).fake::<String>())
        }
        "fake.company.suffix" => {
            mongodb::bson::Bson::String(Suffix(fake::locales::EN).fake::<String>())
        }
        "fake.company.industry" => {
            mongodb::bson::Bson::String(Industry(fake::locales::EN).fake::<String>())
        }

        // CreditCard
        "fake.creditCard.number" => {
            mongodb::bson::Bson::String(CreditCardNumber(fake::locales::EN).fake::<String>())
        }

        // Currency
        "fake.currency.code" => {
            mongodb::bson::Bson::String(CurrencyCode(fake::locales::EN).fake::<String>())
        }
        "fake.currency.name" => {
            mongodb::bson::Bson::String(CurrencyName(fake::locales::EN).fake::<String>())
        }
        "fake.currency.symbol" => {
            mongodb::bson::Bson::String(CurrencySymbol(fake::locales::EN).fake::<String>())
        }

        // FileSystem
        "fake.fileSystem.extension" => {
            mongodb::bson::Bson::String(FileExtension(fake::locales::EN).fake::<String>())
        }
        "fake.fileSystem.fileName" => {
            mongodb::bson::Bson::String(FileName(fake::locales::EN).fake::<String>())
        }
        "fake.fileSystem.filePath" => {
            mongodb::bson::Bson::String(FilePath(fake::locales::EN).fake::<String>())
        }
        "fake.fileSystem.mimeType" => {
            mongodb::bson::Bson::String(MimeType(fake::locales::EN).fake::<String>())
        }

        // Internet
        "fake.internet.domainSuffix" => {
            mongodb::bson::Bson::String(DomainSuffix(fake::locales::EN).fake::<String>())
        }
        "fake.internet.freeEmail" => {
            mongodb::bson::Bson::String(FreeEmail(fake::locales::EN).fake::<String>())
        }
        "fake.internet.ipV4" => {
            mongodb::bson::Bson::String(IPv4(fake::locales::EN).fake::<String>())
        }
        "fake.internet.ipV6" => {
            mongodb::bson::Bson::String(IPv6(fake::locales::EN).fake::<String>())
        }
        "fake.internet.password" => {
            mongodb::bson::Bson::String(Password(fake::locales::EN, 8..12).fake::<String>())
        }
        "fake.internet.email" => {
            mongodb::bson::Bson::String(FreeEmail(fake::locales::EN).fake::<String>())
        }
        "fake.internet.username" => {
            mongodb::bson::Bson::String(Username(fake::locales::EN).fake::<String>())
        }

        // Job
        "fake.job.field" => mongodb::bson::Bson::String(Field(fake::locales::EN).fake::<String>()),
        "fake.job.position" => {
            mongodb::bson::Bson::String(Position(fake::locales::EN).fake::<String>())
        }
        "fake.job.seniority" => {
            mongodb::bson::Bson::String(Seniority(fake::locales::EN).fake::<String>())
        }
        "fake.job.title" => {
            mongodb::bson::Bson::String(JobTitle(fake::locales::EN).fake::<String>())
        }

        // Lorem
        "fake.lorem.sentence" => {
            mongodb::bson::Bson::String(Sentence(fake::locales::EN, 1..5).fake::<String>())
        }
        "fake.lorem.word" => mongodb::bson::Bson::String(Word(fake::locales::EN).fake::<String>()),
        "fake.lorem.paragraph" => {
            mongodb::bson::Bson::String(Paragraph(fake::locales::EN, 1..5).fake::<String>())
        }

        // Name
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

        "fake.number.u8" => mongodb::bson::Bson::Int32((0..=255).fake::<u8>() as i32),
        "fake.number.i32" => mongodb::bson::Bson::Int32((0..=1000).fake::<i32>()),
        "fake.number.u32" => mongodb::bson::Bson::Int64((0..=1000).fake::<u32>() as i64),
        "fake.number.i64" => mongodb::bson::Bson::Int64((0..=1000).fake::<i64>()),

        "fake.random.uuid" => mongodb::bson::Bson::String(fake::uuid::UUIDv4.fake::<String>()),

        method if fake_method.starts_with("ref") => {
            let collection_name = method.split('.').collect::<Vec<&str>>()[1];
            let collection_id = ids_pool.get(collection_name).unwrap();
            let random_id = collection_id.choose(&mut rand::thread_rng()).unwrap();
            mongodb::bson::Bson::ObjectId(*random_id)
        }

        method if fake_method.starts_with("from") => {
            let sequence = method.split('.').collect::<Vec<&str>>()[1];
            let values = sequence.split('|').collect::<Vec<&str>>();
            let random_value = values.choose(&mut rand::thread_rng()).unwrap();
            mongodb::bson::Bson::String(random_value.to_string())
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_generate_fake_value() {
        let ids_pool = HashMap::new();

        assert!(matches!(
            generate_fake_value("fake.name.firstName", &ids_pool),
            mongodb::bson::Bson::String(_)
        ));
        assert!(matches!(
            generate_fake_value("fake.address.city", &ids_pool),
            mongodb::bson::Bson::String(_)
        ));
        assert!(matches!(
            generate_fake_value("fake.number.i32", &ids_pool),
            mongodb::bson::Bson::Int32(_)
        ));
        assert!(matches!(
            generate_fake_value("fake.currency.code", &ids_pool),
            mongodb::bson::Bson::String(_)
        ));
        assert!(matches!(
            generate_fake_value("fake.random.uuid", &ids_pool),
            mongodb::bson::Bson::String(_)
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
