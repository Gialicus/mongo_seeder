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
        "firstName": "fake.name.firstName",
        "lastName": "fake.name.lastName",
        "emails": ["fake.internet.email"],
        "address": {
          "city": "fake.address.city",
          "street": "fake.address.streetName",
          "zipCode": "fake.address.zipCode"
        },
        "friends": [
          {
            "firstName": "fake.name.firstName",
            "lastName": "fake.name.lastName"
          }
        ],
        "age": "fake.number.u8"
      }
    },
    {
      "name": "products",
      "number_of_items": 200,
      "schema": {
        "productName": "fake.name.fullName",
        "price": "fake.number.i32",
        "category": "ref.categories",
        "reviews": [
          {
            "username": "fake.internet.username",
            "rating": "fake.number.u8"
          }
        ]
      }
    },
    {
      "name": "orders",
      "number_of_items": 1000,
      "schema": {
        "orderNumber": "fake.random.uuid",
        "user": "ref.users",
        "status": "from.created|processed|shipped",
        "products": ["ref.products"]
      }
    },
    {
      "name": "categories",
      "number_of_items": 50,
      "schema": {
        "categoryName": "fake.internet.username"
      }
    }
  ]
}
```

## Supported Method

#### Address

```
fake.address.buildingNumber
fake.address.cityName
fake.address.cityPrefix
fake.address.citySuffix
fake.address.countryCode
fake.address.countryName
fake.address.latitude
fake.address.longitude
fake.address.postCode
fake.address.secondaryAddress
fake.address.stateAbbr
fake.address.stateName
fake.address.streetName
fake.address.streetSuffix
fake.address.timeZone
fake.address.zipCode
```

#### Boolean

```
fake.boolean.boolean
```

#### Crono

```
fake.chrono.date
fake.chrono.datetime
fake.chrono.duration
fake.chrono.time
```

#### Company

```
fake.company.bs
fake.company.buzzword
fake.company.catchPhrase
fake.company.name
fake.company.profession
fake.company.suffix
fake.company.industry
```

#### Carta di Credito

```
fake.creditCard.number
```

#### Currency

```
fake.currency.code
fake.currency.name
fake.currency.symbol
```

#### FileSystem

```
fake.fileSystem.extension
fake.fileSystem.fileName
fake.fileSystem.filePath
fake.fileSystem.mimeType
```

#### Internet

```
fake.internet.domainSuffix
fake.internet.freeEmail
fake.internet.ipV4
fake.internet.ipV6
fake.internet.password
fake.internet.email
fake.internet.username
```

#### Job

```
fake.job.field
fake.job.position
fake.job.seniority
fake.job.title
```

#### Lorem

```
fake.lorem.sentence
fake.lorem.word
fake.lorem.paragraph
```

#### Name

```
fake.name.firstName
fake.name.lastName
fake.name.fullName
```

#### Number

```
fake.number.u8
fake.number.i32
fake.number.u32
fake.number.i64
fake.number.u64
fake.number.f32
fake.number.f64
```

#### Uuid

```
fake.random.uuid
```

#### Reference

```
ref.<collection_name>
```

#### Random from list of value

```
from.<value1|value2|...>
```
