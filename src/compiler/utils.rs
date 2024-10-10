use uuid::Uuid;

use crate::types::*;

/// Check if `c` is a hex
pub fn is_hex(c: char) -> bool {
    "0123456789abcdefABCDEF".contains(c)
}

/// UUID generator for stuff
pub fn giv_me_uuid() -> String {
    Uuid::new_v4().to_string().to_uppercase()
}

/// Convert `&Value` to a Hopscotch-readable parameter
pub fn value_to_param(value: &Value, typ: i32, variable: Option<String>) -> Param {
    Param {
        datum: value.datum.to_owned(),
        default_value: "".to_string(),
        key: "".to_string(),
        typ,
        value: value.value.to_owned(),
        variable,
    }
}

/// turn `Vec<Values>` to `Vec<Value>`
pub fn transform_vals(params: Vec<Values>, proj: &Project) -> Vec<Value> {
    params
        .into_iter()
        .map(|v| match v {
            Values::Object(v) => Value {
                value: proj
                    .objects
                    .to_owned()
                    .into_iter()
                    .find(|i| i.name == v)
                    .expect("Object not found")
                    .id,
                datum: None,
            },

            Values::Str(v) => Value {
                value: v,
                datum: None,
            },

            Values::Variable(v, code) => {
                let var = proj
                    .variables
                    .to_owned()
                    .into_iter()
                    .find(|p| p.name == v)
                    .expect("Variable does not exist?")
                    .object_id_string;

                Value {
                    value: "".to_string(),
                    datum: Some(Datum {
                        variable: Some(var),
                        typ: 8000 + code,
                        block_class: None,
                        params: None,
                        object: None,
                    }),
                }
            }

            Values::ObjectVariable(o, v) => {
                //copy-paste
                let var = proj
                    .variables
                    .to_owned()
                    .into_iter()
                    .find(|p| p.name == v)
                    .unwrap()
                    .object_id_string;
                let obj = proj
                    .objects
                    .to_owned()
                    .into_iter()
                    .find(|p| p.name == o)
                    .unwrap()
                    .id;

                Value {
                    value: "".to_string(),
                    datum: Some(Datum {
                        variable: Some(var),
                        typ: 8000,
                        block_class: None,
                        params: None,
                        object: Some(obj),
                    }),
                }
            }

            Values::Conditional(v1, cond, v2) => {
                let id = match cond.as_str() {
                    "==" => 1000,
                    "!=" => 1001,
                    "<" => 1002,
                    ">" => 1003,
                    "and" => 1004,
                    "or" => 1005,
                    ">=" => 1006,
                    "<=" => 1007,
                    &_ => 0,
                };

                Value {
                    value: "".to_string(),
                    datum: Some(Datum {
                        block_class: Some("conditionalOperator".to_string()),
                        object: None,
                        typ: id,
                        variable: None,
                        params: Some(vec![
                            value_to_param(&transform_vals(vec![*v1], proj)[0], 42, None),
                            value_to_param(&transform_vals(vec![*v2], proj)[0], 42, None),
                        ]),
                    }),
                }
            }
        })
        .collect()
}
