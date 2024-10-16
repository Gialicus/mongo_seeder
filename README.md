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

--url: The MongoDB connection URI.
--config: The path to the JSON file containing the schema.

## Configuration

```json
{
  "db": "test_db",
  "collections": [
    {
      "name": "users",
      "number_of_items": 50,
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
      "number_of_items": 20,
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
      "number_of_items": 10,
      "schema": {
        "orderNumber": "fake.random.uuid",
        "user": "ref.users",
        "products": ["ref.products"]
      }
    },
    {
      "name": "categories",
      "number_of_items": 5,
      "schema": {
        "categoryName": "fake.internet.username"
      }
    }
  ]
}
```
