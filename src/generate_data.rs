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
use fake::faker::phone_number::raw::CellNumber;
use fake::Fake;
use mongodb::bson::{oid::ObjectId, Bson};
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::fmt::format;

use crate::error::GenerateDataError;
use crate::types::CollectionConfig;

/// Generates a fake value based on the method string.
fn generate_fake_value(
    fake_method: &str,
    ids_pool: &HashMap<String, Vec<ObjectId>>,
) -> Result<Bson, GenerateDataError> {
    match fake_method {
        // Address
        "address.buildingNumber" => Ok(Bson::String(
            BuildingNumber(fake::locales::EN).fake::<String>(),
        )),
        "address.cityName" => Ok(Bson::String(CityName(fake::locales::EN).fake::<String>())),
        "address.cityPrefix" => Ok(Bson::String(CityPrefix(fake::locales::EN).fake::<String>())),
        "address.citySuffix" => Ok(Bson::String(CitySuffix(fake::locales::EN).fake::<String>())),
        "address.countryCode" => Ok(Bson::String(
            CountryCode(fake::locales::EN).fake::<String>(),
        )),
        "address.countryName" => Ok(Bson::String(
            CountryName(fake::locales::EN).fake::<String>(),
        )),
        "address.latitude" => Ok(Bson::String(Latitude(fake::locales::EN).fake::<String>())),
        "address.longitude" => Ok(Bson::String(Longitude(fake::locales::EN).fake::<String>())),
        "address.postCode" => Ok(Bson::String(PostCode(fake::locales::EN).fake::<String>())),
        "address.secondaryAddress" => Ok(Bson::String(
            SecondaryAddress(fake::locales::EN).fake::<String>(),
        )),
        "address.stateAbbr" => Ok(Bson::String(StateAbbr(fake::locales::EN).fake::<String>())),
        "address.stateName" => Ok(Bson::String(StateName(fake::locales::EN).fake::<String>())),
        "address.streetName" => Ok(Bson::String(StreetName(fake::locales::EN).fake::<String>())),
        "address.streetSuffix" => Ok(Bson::String(
            StreetSuffix(fake::locales::EN).fake::<String>(),
        )),
        "address.timeZone" => Ok(Bson::String(TimeZone(fake::locales::EN).fake::<String>())),
        "address.zipCode" => Ok(Bson::String(ZipCode(fake::locales::EN).fake::<String>())),

        // Boolean
        "boolean.boolean" => Ok(Bson::Boolean(Boolean(fake::locales::EN, 1).fake())),

        // Chrono
        "chrono.date" => Ok(Bson::String(
            fake::faker::chrono::raw::Date(fake::locales::EN).fake::<String>(),
        )),
        "chrono.datetime" => Ok(Bson::String(
            fake::faker::chrono::raw::DateTime(fake::locales::EN).fake::<String>(),
        )),
        "chrono.duration" => Ok(Bson::String(
            fake::faker::number::raw::NumberWithFormat(fake::locales::EN, "###").fake::<String>(),
        )),
        "chrono.time" => Ok(Bson::String(
            fake::faker::chrono::raw::Time(fake::locales::EN).fake::<String>(),
        )),

        // Company
        "company.bs" => Ok(Bson::String(Bs(fake::locales::EN).fake::<String>())),
        "company.buzzword" => Ok(Bson::String(Buzzword(fake::locales::EN).fake::<String>())),
        "company.catchPhrase" => Ok(Bson::String(
            CatchPhrase(fake::locales::EN).fake::<String>(),
        )),
        "company.name" => Ok(Bson::String(
            CompanyName(fake::locales::EN).fake::<String>(),
        )),
        "company.profession" => Ok(Bson::String(Profession(fake::locales::EN).fake::<String>())),
        "company.suffix" => Ok(Bson::String(Suffix(fake::locales::EN).fake::<String>())),
        "company.industry" => Ok(Bson::String(Industry(fake::locales::EN).fake::<String>())),

        // CreditCard
        "creditCard.number" => Ok(Bson::String(
            CreditCardNumber(fake::locales::EN).fake::<String>(),
        )),

        // Currency
        "currency.code" => Ok(Bson::String(
            CurrencyCode(fake::locales::EN).fake::<String>(),
        )),
        "currency.name" => Ok(Bson::String(
            CurrencyName(fake::locales::EN).fake::<String>(),
        )),
        "currency.symbol" => Ok(Bson::String(
            CurrencySymbol(fake::locales::EN).fake::<String>(),
        )),

        // FileSystem
        "fileSystem.extension" => Ok(Bson::String(
            FileExtension(fake::locales::EN).fake::<String>(),
        )),
        "fileSystem.fileName" => Ok(Bson::String(FileName(fake::locales::EN).fake::<String>())),
        "fileSystem.filePath" => Ok(Bson::String(FilePath(fake::locales::EN).fake::<String>())),
        "fileSystem.mimeType" => Ok(Bson::String(MimeType(fake::locales::EN).fake::<String>())),

        // Internet
        "internet.domainSuffix" => Ok(Bson::String(
            DomainSuffix(fake::locales::EN).fake::<String>(),
        )),
        "internet.freeEmail" => Ok(Bson::String(FreeEmail(fake::locales::EN).fake::<String>())),
        "internet.ipV4" => Ok(Bson::String(IPv4(fake::locales::EN).fake::<String>())),
        "internet.ipV6" => Ok(Bson::String(IPv6(fake::locales::EN).fake::<String>())),
        "internet.password" => Ok(Bson::String(
            Password(fake::locales::EN, 8..12).fake::<String>(),
        )),
        "internet.email" => Ok(Bson::String(FreeEmail(fake::locales::EN).fake::<String>())),
        "internet.username" => Ok(Bson::String(Username(fake::locales::EN).fake::<String>())),

        // Job
        "job.field" => Ok(Bson::String(Field(fake::locales::EN).fake::<String>())),
        "job.position" => Ok(Bson::String(Position(fake::locales::EN).fake::<String>())),
        "job.seniority" => Ok(Bson::String(Seniority(fake::locales::EN).fake::<String>())),
        "job.title" => Ok(Bson::String(JobTitle(fake::locales::EN).fake::<String>())),

        // Lorem
        "lorem.sentence" => Ok(Bson::String(
            Sentence(fake::locales::EN, 1..5).fake::<String>(),
        )),
        "lorem.word" => Ok(Bson::String(Word(fake::locales::EN).fake::<String>())),
        "lorem.paragraph" => Ok(Bson::String(
            Paragraph(fake::locales::EN, 1..5).fake::<String>(),
        )),

        // Name
        "name.firstName" => Ok(Bson::String(FirstName(fake::locales::EN).fake::<String>())),
        "name.lastName" => Ok(Bson::String(LastName(fake::locales::EN).fake::<String>())),
        "name.fullName" => Ok(Bson::String(format!(
            "{} {}",
            FirstName(fake::locales::EN).fake::<String>(),
            LastName(fake::locales::EN).fake::<String>()
        ))),

        "number.u8" => Ok(Bson::Int32((0..=255).fake::<u8>() as i32)),
        "number.i32" => Ok(Bson::Int32((0..=1000).fake::<i32>())),
        "number.u32" => Ok(Bson::Int64((0..=1000).fake::<u32>() as i64)),
        "number.i64" => Ok(Bson::Int64((0..=1000).fake::<i64>())),
        "number.u64" => Ok(Bson::Int64((0..=1000).fake::<u64>() as i64)),
        "number.f32" => Ok(Bson::Double((0.0..1000.0).fake::<f32>() as f64)),
        "number.f64" => Ok(Bson::Double((0.0..=1000.0).fake::<f64>())),
        "number.phoneNumber" => Ok(Bson::String(format!(
            "{}",
            CellNumber(fake::locales::EN).fake::<String>()
        ))),

        "random.uuid" => Ok(Bson::String(fake::uuid::UUIDv4.fake::<String>())),

        method if fake_method.starts_with("ref") => {
            let collection_name = method.split('.').collect::<Vec<&str>>()[1];
            let collection_id = ids_pool.get(collection_name).ok_or_else(|| {
                GenerateDataError::CollectionNotFound(collection_name.to_string())
            })?;
            let random_id = collection_id
                .choose(&mut rand::thread_rng())
                .ok_or_else(|| {
                    GenerateDataError::RandomIdSelectionFailed(collection_name.to_string())
                })?;
            Ok(Bson::ObjectId(*random_id))
        }

        method if fake_method.starts_with("from") => {
            let sequence = method.split('.').collect::<Vec<&str>>()[1];
            let values = sequence.split('|').collect::<Vec<&str>>();
            let random_value = values
                .choose(&mut rand::thread_rng())
                .ok_or_else(|| GenerateDataError::RandomIdSelectionFailed(sequence.to_string()))?;
            Ok(Bson::String(random_value.to_string()))
        }

        _ => Err(GenerateDataError::InvalidFakeMethod(
            fake_method.to_string(),
        )),
    }
}

/// Generates data based on a schema, handling scalar fields, arrays, and nested objects.
pub fn generate_data(
    schema: &HashMap<String, serde_json::Value>,
    number_of_children: usize,
    ids_pool: &HashMap<String, Vec<ObjectId>>,
) -> Result<Bson, GenerateDataError> {
    let mut document = mongodb::bson::Document::new();

    for (key, value) in schema.iter() {
        let generated_value = match value {
            // Scalar value represented as a string
            serde_json::Value::String(fake_method) => generate_fake_value(fake_method, ids_pool)?,

            // Array of values or objects
            serde_json::Value::Array(array_spec) => {
                if array_spec.is_empty() {
                    Bson::Array(vec![]) // Empty array if the schema contains no information
                } else if let Some(serde_json::Value::String(fake_method)) = array_spec.get(0) {
                    // Array of primitive values (e.g., array of strings)
                    let generated_array: Vec<Bson> = (0..number_of_children)
                        .map(|_| generate_fake_value(fake_method, ids_pool))
                        .collect::<Result<_, _>>()?;
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
                        .collect::<Result<_, _>>()?;
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
                )?;
                Bson::Document(nested_document.as_document().unwrap().clone())
            }
            _ => Bson::Null,
        };

        document.insert(key.clone(), generated_value);
    }

    Ok(Bson::Document(document))
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
            generate_fake_value("name.firstName", &ids_pool),
            Ok(Bson::String(_))
        ));
        assert!(matches!(
            generate_fake_value("address.cityName", &ids_pool),
            Ok(Bson::String(_))
        ));
        assert!(matches!(
            generate_fake_value("number.i32", &ids_pool),
            Ok(Bson::Int32(_))
        ));
        assert!(matches!(
            generate_fake_value("currency.code", &ids_pool),
            Ok(Bson::String(_))
        ));
        assert!(matches!(
            generate_fake_value("random.uuid", &ids_pool),
            Ok(Bson::String(_))
        ));
    }

    #[test]
    fn test_generate_data() {
        let schema = json!({
            "first_name": "name.firstName",
            "last_name": "name.lastName",
            "address": {
                "street": "address.streetName",
                "city": "address.cityName"
            },
            "emails": ["internet.email"],
            "numbers": ["number.i32"]
        });

        let ids_pool = HashMap::new();
        let schema_map: HashMap<String, serde_json::Value> =
            schema.as_object().unwrap().clone().into_iter().collect();
        let generated_data = generate_data(&schema_map, 3, &ids_pool).unwrap();

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
