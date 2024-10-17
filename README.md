# Mongo Seeder CLI

This project is a CLI written in Rust to populate a MongoDB database with sample data based on schema.

## Installation

Make sure you have [Rust](https://www.rust-lang.org/) installed. Then, clone the repository and build the project:

```sh
git clone https://github.com/gialicus/mongo_seeder.git
cd mongo_seeder
cargo build --release
```

## Usage

Run the following command to start the seeder:

```sh
./target/release/mongo_seeder --url <MONGO_URL> --config <FILE_PATH>
```

## Options

```
--url: The MongoDB connection URI.
--config: The path to the JSON file containing the schema.
```

## Configuration Example

```json
{
  "db": "test_db",
  "collections": [
    {
      "name": "users",
      "number_of_items": 500,
      "number_of_children": 5,
      "schema": {
        "firstName": "name.firstName",
        "lastName": "name.lastName",
        "emails": ["internet.email"],
        "address": {
          "city": "address.city",
          "street": "address.streetName",
          "zipCode": "address.zipCode"
        },
        "friends": [
          {
            "firstName": "name.firstName",
            "lastName": "name.lastName"
          }
        ],
        "age": "number.u8"
      }
    },
    {
      "name": "products",
      "number_of_items": 200,
      "schema": {
        "productName": "name.fullName",
        "price": "number.i32",
        "category": "ref.categories",
        "reviews": [
          {
            "username": "internet.username",
            "rating": "number.u8"
          }
        ]
      }
    },
    {
      "name": "orders",
      "number_of_items": 1000,
      "schema": {
        "orderNumber": "random.uuid",
        "user": "ref.users",
        "status": "from.created|processed|shipped",
        "products": ["ref.products"]
      }
    },
    {
      "name": "categories",
      "number_of_items": 50,
      "schema": {
        "categoryName": "internet.username"
      }
    }
  ]
}
```

## Supported Method

#### Address

```
address.buildingNumber
address.cityName
address.cityPrefix
address.citySuffix
address.countryCode
address.countryName
address.latitude
address.longitude
address.postCode
address.secondaryAddress
address.stateAbbr
address.stateName
address.streetName
address.streetSuffix
address.timeZone
address.zipCode
```

#### Boolean

```
boolean.boolean
```

#### Crono

```
chrono.date
chrono.datetime
chrono.duration
chrono.time
```

#### Company

```
company.bs
company.buzzword
company.catchPhrase
company.name
company.profession
company.suffix
company.industry
```

#### Carta di Credito

```
creditCard.number
```

#### Currency

```
currency.code
currency.name
currency.symbol
```

#### FileSystem

```
fileSystem.extension
fileSystem.fileName
fileSystem.filePath
fileSystem.mimeType
```

#### Internet

```
internet.domainSuffix
internet.freeEmail
internet.ipV4
internet.ipV6
internet.password
internet.email
internet.username
```

#### Job

```
job.field
job.position
job.seniority
job.title
```

#### Lorem

```
lorem.sentence
lorem.word
lorem.paragraph
```

#### Name

```
name.firstName
name.lastName
name.fullName
```

#### Number

```
number.u8
number.i32
number.u32
number.i64
number.u64
number.f32
number.f64
```

#### Uuid

```
random.uuid
```

#### Reference

```
ref.<collection_name>
```

#### Random from list of value

```
from.<value1|value2|...>
```
