use fake::faker::address::raw::*;
use fake::faker::company::raw::*;
use fake::faker::currency::raw::*;
use fake::faker::internet::raw::*;
use fake::faker::name::raw::*;
use fake::Fake;
use std::collections::HashMap;

/// Genera un valore finto in base alla stringa del metodo.
fn generate_fake_value(fake_method: &str) -> serde_json::Value {
    match fake_method {
        "fake.name.firstName" => {
            serde_json::Value::String(FirstName(fake::locales::EN).fake::<String>())
        }
        "fake.name.lastName" => {
            serde_json::Value::String(LastName(fake::locales::EN).fake::<String>())
        }
        "fake.name.fullName" => serde_json::Value::String(format!(
            "{} {}",
            FirstName(fake::locales::EN).fake::<String>(),
            LastName(fake::locales::EN).fake::<String>()
        )),

        "fake.address.streetName" => {
            serde_json::Value::String(StreetName(fake::locales::EN).fake::<String>())
        }
        "fake.address.city" => {
            serde_json::Value::String(CityName(fake::locales::EN).fake::<String>())
        }
        "fake.address.zipCode" => {
            serde_json::Value::String(ZipCode(fake::locales::EN).fake::<String>())
        }

        "fake.internet.email" => {
            serde_json::Value::String(FreeEmail(fake::locales::EN).fake::<String>())
        }
        "fake.internet.username" => {
            serde_json::Value::String(Username(fake::locales::EN).fake::<String>())
        }

        "fake.company.name" => {
            serde_json::Value::String(CompanyName(fake::locales::EN).fake::<String>())
        }
        "fake.company.suffix" => {
            serde_json::Value::String(CompanySuffix(fake::locales::EN).fake::<String>())
        }

        "fake.number.u8" => serde_json::Value::Number((0..=255).fake::<u8>().into()),
        "fake.number.i32" => serde_json::Value::Number((0..=1000).fake::<i32>().into()),
        "fake.number.u32" => serde_json::Value::Number((0..=1000).fake::<u32>().into()),
        "fake.number.i64" => serde_json::Value::Number((0..=1000).fake::<i64>().into()),

        "fake.currency.code" => {
            serde_json::Value::String(CurrencyCode(fake::locales::EN).fake::<String>())
        }
        "fake.currency.name" => {
            serde_json::Value::String(CurrencyName(fake::locales::EN).fake::<String>())
        }
        "fake.currency.symbol" => {
            serde_json::Value::String(CurrencySymbol(fake::locales::EN).fake::<String>())
        }

        "fake.chrono.datetime" => serde_json::Value::String(
            fake::faker::chrono::raw::DateTime(fake::locales::EN).fake::<String>(),
        ),
        "fake.chrono.date" => serde_json::Value::String(
            fake::faker::chrono::raw::Date(fake::locales::EN).fake::<String>(),
        ),
        "fake.chrono.duration" => serde_json::Value::String(
            fake::faker::number::raw::NumberWithFormat(fake::locales::EN, "###").fake::<String>(),
        ),

        _ => serde_json::Value::String("unsupported_method".to_string()),
    }
}

/// Genera dati in base a uno schema, gestendo campi scalari, array e oggetti annidati.
pub fn generate_data(
    schema: &HashMap<String, serde_json::Value>,
    number_of_children: usize,
) -> serde_json::Value {
    let mut document = serde_json::Map::new();

    for (key, value) in schema.iter() {
        let generated_value = match value {
            // Valore scalare rappresentato come stringa
            serde_json::Value::String(fake_method) => generate_fake_value(fake_method),

            // Array di valori o di oggetti
            serde_json::Value::Array(array_spec) => {
                if array_spec.is_empty() {
                    serde_json::Value::Array(vec![]) // Array vuoto se lo schema non contiene informazioni
                } else if let Some(serde_json::Value::String(fake_method)) = array_spec.get(0) {
                    // Array di valori primitivi (es. array di stringhe)
                    let generated_array: Vec<serde_json::Value> = (0..number_of_children)
                        .map(|_| generate_fake_value(fake_method))
                        .collect();
                    serde_json::Value::Array(generated_array)
                } else if let Some(serde_json::Value::Object(object_schema)) = array_spec.get(0) {
                    // Array di oggetti: ogni oggetto segue uno schema
                    let generated_array: Vec<serde_json::Value> = (0..number_of_children)
                        .map(|_| {
                            generate_data(
                                &object_schema
                                    .iter()
                                    .map(|(k, v)| (k.clone(), v.clone()))
                                    .collect(),
                                number_of_children,
                            )
                        })
                        .collect();
                    serde_json::Value::Array(generated_array)
                } else {
                    serde_json::Value::Array(vec![])
                }
            }
            // Oggetti annidati
            serde_json::Value::Object(nested_schema) => {
                let nested_document = generate_data(
                    &nested_schema
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                    number_of_children,
                );
                serde_json::Value::Object(nested_document.as_object().unwrap().clone())
            }
            _ => serde_json::Value::Null,
        };

        document.insert(key.clone(), generated_value);
    }

    serde_json::Value::Object(document)
}
