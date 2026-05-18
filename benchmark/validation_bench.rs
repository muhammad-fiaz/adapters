#![feature(test)]

extern crate test;

use adapters::{Adapt, Adapter, Error, Pipeline, SchemaProvider, SchemaValidator, Validate, Value};
use adapters_macros::Schema as SchemaDerived;
use std::collections::BTreeMap;
use test::Bencher;

// professionally expert
#[derive(SchemaDerived, Debug)]
struct StringModel {
    #[schema(min_length = 3, max_length = 20)]
    username: String,
    #[schema(email)]
    email: String,
    #[schema(alphanumeric)]
    code: String,
    #[schema(non_empty)]
    bio: String,
}

#[bench]
fn bench_string_validation_passes(b: &mut Bencher) {
    let json = r#"{"username": "rustacean", "email": "rust@example.com", "code": "rust123", "bio": "Rust developer"}"#;
    b.iter(|| {
        let instance = StringModel::from_json(test::black_box(json)).unwrap();
        let _ = test::black_box(instance.validate());
    });
}

#[bench]
fn bench_string_validation_fails(b: &mut Bencher) {
    let json = r#"{"username": "ru", "email": "invalid-email", "code": "rust_123!", "bio": ""}"#;
    b.iter(|| {
        let _ = test::black_box(StringModel::from_json(json));
    });
}

// professionally expert
#[derive(SchemaDerived, Debug)]
struct NumberModel {
    #[schema(positive)]
    pos_int: i32,
    #[schema(negative)]
    neg_float: f64,
    #[schema(non_zero)]
    nonzero: i64,
    #[schema(min = 10, max = 100)]
    range_int: u8,
}

#[bench]
fn bench_number_validation_passes(b: &mut Bencher) {
    let json = r#"{"pos_int": 42, "neg_float": -3.15, "nonzero": -100, "range_int": 50}"#;
    b.iter(|| {
        let instance = NumberModel::from_json(test::black_box(json)).unwrap();
        let _ = test::black_box(instance.validate());
    });
}

// professionally expert
#[derive(SchemaDerived, Debug)]
struct StrictModel {
    #[schema(strict)]
    strict_int: i32,
    #[schema(strict)]
    strict_bool: bool,
}

#[derive(SchemaDerived, Debug)]
struct CoercedModel {
    coerced_int: i32,
    coerced_bool: bool,
}

#[bench]
fn bench_strict_mode_validation(b: &mut Bencher) {
    let json = r#"{"strict_int": 42, "strict_bool": true}"#;
    b.iter(|| {
        let instance = StrictModel::from_json(test::black_box(json)).unwrap();
        let _ = test::black_box(instance.validate());
    });
}

#[bench]
fn bench_coercion_validation(b: &mut Bencher) {
    let value = Value::Object(BTreeMap::from([
        ("coerced_int".to_string(), Value::String("42".into())),
        ("coerced_bool".to_string(), Value::Int(1)),
    ]));
    let schema = CoercedModel::schema();
    b.iter(|| {
        let _ = test::black_box(schema.validate(test::black_box(&value), "root"));
    });
}

// professionally expert
#[derive(SchemaDerived, Debug, Clone)]
struct Address {
    city: String,
    country: String,
}

#[derive(SchemaDerived, Debug, Clone)]
struct UserWithAddress {
    username: String,
    address: Address,
}

#[bench]
fn bench_nested_models_validation(b: &mut Bencher) {
    let json = r#"{"username": "alice", "address": {"city": "New York", "country": "USA"}}"#;
    b.iter(|| {
        let instance = UserWithAddress::from_json(test::black_box(json)).unwrap();
        let _ = test::black_box(instance.validate());
    });
}

// professionally expert
#[derive(SchemaDerived, Debug)]
struct OptionalDefaultModel {
    #[schema(optional)]
    nickname: Option<String>,
    #[schema(default = "India")]
    country: String,
    #[schema(default = 100)]
    score: i32,
}

#[bench]
fn bench_optional_defaults_validation(b: &mut Bencher) {
    let json = r#"{"nickname": null}"#;
    b.iter(|| {
        let instance = OptionalDefaultModel::from_json(test::black_box(json)).unwrap();
        let _ = test::black_box(instance.validate());
    });
}

// professionally expert
struct UserRow {
    id: i64,
    username: String,
    db_hash: String,
}

#[derive(Debug)]
struct UserResponse {
    id: i64,
    username: String,
}

impl Adapt<UserRow> for UserResponse {
    fn adapt(row: UserRow) -> Result<Self, Error> {
        Ok(UserResponse {
            id: row.id,
            username: row.username,
        })
    }
}

#[bench]
fn bench_model_adaptation(b: &mut Bencher) {
    b.iter(|| {
        let row = UserRow {
            id: test::black_box(999),
            username: test::black_box("db_user".to_string()),
            db_hash: test::black_box("bcrypt_hash_placeholder".to_string()),
        };
        let _ = test::black_box(UserResponse::adapt(row));
    });
}

#[bench]
fn bench_pipeline_transformation(b: &mut Bencher) {
    let pipeline = Pipeline::new()
        .step(|val| match val {
            Value::Int(n) => Ok(Value::Int(n * 2)),
            other => Ok(other),
        })
        .step(|val| match val {
            Value::Int(n) => Ok(Value::Int(n + 1)),
            other => Ok(other),
        });

    b.iter(|| {
        let val = Value::Int(test::black_box(50));
        let _ = test::black_box(pipeline.run(val));
    });
}
