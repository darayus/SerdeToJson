extern crate serde;
extern crate serde_json;

use serde::ser::Serialize;
use serde_json::{Value, to_value};

pub trait ToJson {
    fn to_json(&self) -> Value;
}

impl<T> ToJson for T where T: Serialize {
    fn to_json(&self) -> Value {
        to_value(&self) 
    }
}

#[test]
fn serde_to_json_str() {
    let my_text = "My Text".to_owned();
    assert_eq!(my_text.to_json(), Value::String(my_text.clone()));
}

#[test]
fn serde_to_json_vex() {
    let my_vec = vec!["This", "text"];
    let my_vec_expected = vec!["This".to_json(), "text".to_json()];
    assert_eq!(my_vec.to_json(), Value::Array(my_vec_expected));
}
