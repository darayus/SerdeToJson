extern crate serde;
extern crate serde_json;

use serde::ser::Serialize;
use serde_json::{Value, to_value};

pub trait ToJson: Serialize {
    fn to_json(&self) -> Value {
        to_value(&self) 
    }
}

impl<T> ToJson for T where T: Serialize {}

#[test]
fn serde_to_json() {
    let my_text = "My Text".to_owned();
    assert_eq!(my_text.to_json(), Value::String(my_text.clone()));
}
