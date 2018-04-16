#![feature(test)]

extern crate test;
#[macro_use]
extern crate stringly_typed;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde;

use test::Bencher;
use stringly_typed::{StringlyTyped, Value};

fn test_fixture() -> Outer {
    Outer {
        inner: Inner {
            x: 3.14,
            y: 42,
            key_value_pair: KeyValue {
                key: String::from("Key"),
                value: String::from("Value"),
            },
        }
    }
}

#[bench]
fn static_assign(b: &mut Bencher) {
    let original = test_fixture();

    b.iter(|| {
        let mut new = original.clone();
        new.inner.key_value_pair.key = String::from("new");
        new
    });
}

#[bench]
fn stringly_update(b: &mut Bencher) {
    let original = test_fixture();

    b.iter(|| {
        let mut new = original.clone();
        let key = "inner.key_value_pair.key";
        let value = "new";
        new.set_value(key.split("."), Value::from(value)).unwrap();
        new
    });
}

#[bench]
fn serialize_deserialize(b: &mut Bencher) {
    let original = test_fixture();

    b.iter(|| {
        let new = original.clone();
        let mut serialized = serde_json::to_value(new).unwrap();

        let new_value = "new";
        serialized["inner"]["key_value_pair"]["key"] = new_value.into();

        let deserialized: Outer = serde_json::from_value(serialized).unwrap();
        deserialized
    });
}


#[derive(Debug, Clone, PartialEq, StringlyTyped, Serialize, Deserialize)]
struct Outer {
  inner: Inner,
}

#[derive(Debug, Clone, PartialEq, StringlyTyped, Serialize, Deserialize)]
struct Inner {
  x: f64,
  y: i64,
  key_value_pair: KeyValue,
}

#[derive(Debug, Clone, PartialEq, StringlyTyped, Serialize, Deserialize)]
struct KeyValue {
    key: String,
    value: String,
}